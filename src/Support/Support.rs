use crate::Config::Configuration::{LatencyChecker, LatencyCheckerOutputList, TokenApiResponse};
use std::env;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use regex::Regex;
use reqwest::blocking::{Client, RequestBuilder};
use chrono::{Utc, TimeZone};
use cron::Schedule;
use std::str::FromStr;
use std::fmt;
use prettytable::{Table, Row, Cell, format};
use serde_json;
use serde_yaml;

/*=========================================================
 *  LatencyChecker
 *=========================================================
 * проверяет операции связанные с проверкой задержек
 * с различных местоположений включает функции
 * для олучения токена отправки запроса проверки
 * задержек и выборки минимальных задержек
 *
 */
#[allow(non_snake_case)]
impl LatencyChecker {

    /*=========================================================
     * DoGetTokenRequest(&self) -> Result<i32, Box<dyn Error>>
     *=========================================================
     * выполняет GET-запрос к API для получения токена
     * доступа и количевство доступных запросов
     * если ApiKey не установлен то возвращает
     * ошибку проверяет статус ответа если он не
     * '200 OK' возвращает ошибку
     *
     */
    #[allow(non_snake_case)]
    pub fn DoGetTokenRequest(&self) -> Result<i32, Box<dyn Error>> {
        if self.ApiKey == "NOT_SET" {
            return Err("TAXONIM_X_API_KEY Env Var Not Set".into());
        }
        let Client = Client::new();
        let Request = Client.get(&self.ServiceApiTokensUrl).header("X_API_KEY", &self.ApiKey).send()?;

        if Request.status() != reqwest::StatusCode::OK {
            return Err(format!("Status Code Received: {} ...But Status Code Expected: 200", Request.status()).into());
        }
        let Response: TokenApiResponse = Request.json()?;

        Ok(Response.RequestCount as i32)
    }

    /*=====================================================================================================================
     *DoPostLatencyCheckRequest(&self, Locations: Vec<String>) -> Result<HashMap<String, f64>, Box<dyn Error>>
     *=====================================================================================================================
     * выполняет POST-запрос к API отправляя целевой URL и список местоположений
     * для проверки задержки проверяет статус ответа если он не '200 OK'
     * возвращает ошибку
     *
     */
    #[allow(non_snake_case)]
    pub fn DoPostLatencyCheckRequest(&self, Locations: Vec<String>) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let CLient = Client::new();
        let RequestBody = serde_json::json!({
            "Target": self.TargetURL,
            "Locations": Locations,
        });
        let Response = CLient.post(&self.ServiceApiUrl).header("Content-Type", "application/json").header("X_API_KEY", &self.ApiKey).json(&RequestBody).send()?;

        if Response.status() != reqwest::StatusCode::OK {
            return Err(format!("Status Code Received: {} ...But Status Code Expected: 200", Response.status()).into());
        }
        let Response: HashMap<String, f64> = Response.json()?;
        Ok(Response)
    }

    /*=================================================================================================
     *GetMinimumLatencies(&self, Latencies: HashMap<String, f64>) -> (Vec<String>, Vec<f64>)
     *=================================================================================================
     * возврашает списко местоположений с минимальными задержками
     * ограниченый количеством 'OutputLocationsNumber'
     * сортирует задержки по возрастанию и выбирает 'OutputLocationsNumber'
     * местоположение с наименьшими задержками
     *
     */
    #[allow(non_snake_case)]
    pub fn GetMinimumLatencies(&self, Latencies: HashMap<String, f64>) -> (Vec<String>, Vec<f64>) {
        let mut SortedLatencies: Vec<_> = Latencies.iter().collect();
        SortedLatencies.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        let OutputLen = self.OutputLocationsNumber.min(SortedLatencies.len() as isize);
        let mut Keys = Vec::with_capacity(OutputLen as usize);
        let mut Values = Vec::with_capacity(OutputLen as usize);

        for (Key, Value) in SortedLatencies.iter().take(OutputLen as usize) {
            Keys.push(Key.to_string());
            Values.push(*Value)
        }
        (Keys, Values)
    }
}


/*=========================================================
 *  LatencyCheckerOutputList
 *=========================================================
 * Структура Содержащая результаты проверок Latency
 * для различных локации эти результаты можно
 * вывести можно вывести в виде таблицы
 * форматировать как JSON или YAML для удобства чтения
 * или дальнейшей обработки
 *
 */
impl LatencyCheckerOutputList {


    /*========================================================================
     *      WriteOutputTable(&self)
     *========================================================================
     * метод форматирует и выводит данные в табличном формате
     * табилца содержит два столбца "Location" (местоположение)
     и "Average Latency"  средняя задержка).
     * первый элемент списка выводит элементы с текстом
     * чтобы выделить его (предполагается, что это наилучшее местоположение).
     *
     */
    #[allow(non_snake_case)]
    fn WriteOutputTable(&self) {
        let mut Table = Table::new();
        Table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        Table.set_titles(Row::new(vec![
            Cell::new("Location").style_spec("Fw"),
            Cell::new("Average Latency").style_spec("Fw")
        ]));

        for (I, Result) in self.Result.iter().enumerate() {
            let LocationCell = if I == 0 {
                Cell::new(&Result.Locations).style_spec("Fg")
            } else {
                Cell::new(&Result.Locations).style_spec("Fg")
            };

            let LatencyCell = if I == 0 {
                Cell::new(&format!("{:.2}", Result.AvgLatency)).style_spec("Fg")
            } else {
                Cell::new(&format!("{:.2}", Result.AvgLatency)).style_spec("Fg")
            };
            Table.add_row(Row::new(vec![LocationCell, LatencyCell]));
        }
        Table.printstd();
    }


    /*========================================================================
     *      WriteOutputJson(&self)
     *========================================================================
     * Метод сериализует данные структуры `LatencyCheckerOutputList` в формат JSON
     * с отступами для удобства чтения. В случае ошибки сериализации выводит сообщение
     * об ошибке.
     *
     */
    #[allow(non_snake_case)]
    fn WriteOutputJson(&self) {
        match serde_json::to_string_pretty(&self) {
            Ok(JsonOutput) => println!("{}", JsonOutput),
            Err(E) => eprintln!("[ FAILED ]: To Serialize To JSON: {}", E),
        }
    }


    /*========================================================================
     *      WriteOutputYaml(&self)
     *========================================================================
     * Метод сериализует данные структуры `LatencyCheckerOutputList` в формат YAML
     * для более упрощенного текстового представления. В случае ошибки
     * выводит сообщение об ошибке.
     *
     */
    #[allow(non_snake_case)]
    fn WriteOutputYaml(&self) {
        match serde_yaml::to_string(&self) {
            Ok(YamlOutput) => println!("{}", YamlOutput),
            Err(E) => eprintln!("[ FAILED ]: To Serialize To YAML: {}", E),
        }
    }
}





/*=========================================================
 *  GetEnv(Key: &str, Value: &str) -> String
 *=========================================================
 * функция ищет переменнуж окруженния по имени 'Key'
 * если переменная окруженния сущетсвет то возвращает
 * ее значение если переменная не найдена
 * то возвращает 'Value'
 *
 */
#[allow(non_snake_case)]
pub fn GetEnv(Key: &str, Value: &str) -> String {
    env::var(Key).unwrap_or_else(|_| Value.to_string())
}


/*=========================================================
 *  ValidateUrl(InputUrl: &str) -> bool
 *=========================================================
 * проверят является ли 'InputUrl' коректным url
 * если url коректен и содержит хост возврашает true
 * в противном случае возврашает false
 *
 */
#[allow(non_snake_case)]
pub fn ValidateUrl(InputUrl: &str) -> bool {
    let parsed = reqwest::Url::parse(InputUrl);
    parsed.is_ok() && parsed.unwrap().has_host()
}

/*=========================================================
 *  ValidateIntervalTime(Interval: &str) -> bool
 *=========================================================
 * проверяет соответсвует ли 'Interval' формату
 * временного интервала поддерживаемые форматы
 * цело число ха которым следует 's', 'm', 'h'
 * "10s" (10 секунд), "5m" (5 минут), "1h" (1 час).
 *
 */
#[allow(non_snake_case)]
pub fn ValidateIntervalTime(Interval: &str) -> bool {
    let Re = Regex::new(r"^(\d+)(s|m|h)$").unwrap();
    Re.is_match(Interval)
}

/*=========================================================
 *  ValidateCronTime(CronTime: &str) -> bool
 *=========================================================
 * проверяет является ли 'CronTIme' валидным
 * cron-выражением
 *
 */
#[allow(non_snake_case)]
pub fn ValidateCronTime(CronTime: &str) -> bool {
    Schedule::from_str(CronTime).is_ok()
}


/*=========================================================
 *  GetNextTimeCronTime(CronTime: &str) -> i64
 *=========================================================
 * вычесляет оставшейся время до следующего
 * выполнения задачи на основе cron выражения
 *
 */
#[allow(non_snake_case)]
pub fn GetNextTimeCronTime(CronTime: &str) -> i64 {
    let Schedul = match Schedule::from_str(CronTime) {
        Ok(S) => S,
        Err(_) => return -1,
    };
    let NextTime = match Schedul.upcoming(Utc).next() {
        Some(Time) => Time,
        None => return -1,
    };
    let Duration = NextTime.timestamp() - Utc::now().timestamp();
    Duration
}


/*=========================================================
 *  IntervalTimeToSeconds(Interval: &str) -> i64
 *=========================================================
 * преобразует временной интервал 'Interval'
 * в секунды Поддерживаемые форматы
 * "s" для секунд
 * "m" для минут
 * "h" для часов
 * "Interval" строка для временного интервала
 * если формат интервала не верный возврашаем -1
 *
 */
#[allow(non_snake_case)]
pub fn IntervalTimeToSeconds(Interval: &str) -> i64 {
    let Re = Regex::new(r"^(\d+)(s|m|h)$").unwrap();
    if let Some(Captures) = Re.captures(Interval) {
        let TimeValue = Captures[1].parse::<i64>().unwrap_or(-1);
        let TimeUnit = &Captures[2];
        
        match TimeUnit {
            "s" => TimeValue,
            "m" => TimeValue * 60,
            "h" => TimeValue * 3600,
            _ =>-1,
        }
    } else {
        -1
    }
}



/*=========================================================
 *  IntervalTimeToSeconds(Interval: &str) -> i64
 *=========================================================
 * преобразует временной интервал 'Interval'
 * в секунды Поддерживаемые форматы
 * "s" для секунд
 * "m" для минут
 * "h" для часов
 * "Interval" строка для временного интервала
 * если формат интервала не верный возврашаем -1
 *
 */


















