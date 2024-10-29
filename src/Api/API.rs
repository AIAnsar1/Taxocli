use crate::Config::Configuration::{LatencyChecker, LatencyCheckerOutput, LatencyCheckerOutputList, TAXOCLI_API_THROTTLER_TIME, TAXOCLI_LATENCY_TOKEN_COST};
use crate::Latency::Latencychecker::GetMutators;
use std::{collections::HashMap, error::Error, thread::sleep, time::Duration};

impl LatencyChecker {


    /*====================================================================================
     *RunCommandExec(&mut self) -> Result<LatencyCheckerOutputList, Box<dyn Error>>
     *====================================================================================
     * проверяет достаточно ли токенов для выполнения всех запросов
     * выполняет запросы для проверки задержек обрабатывает запросы
     * ответы и возвращает список результатов задержки
     * для различных локаций
     *
     */
    #[allow(non_snake_case)]
    fn RunCommandExec(&mut self) -> Result<LatencyCheckerOutputList, Box<dyn Error>> {

        // Проверяем доступных токенов с помошью метода 'DoGetTokenRequest'
        let AvailableTokens = match self.DoGetTokenRequest() {
            Ok(Tokens) => Tokens,
            Err(ErrCode) => {
                // обработка различных типов ошибок если 'DoGetTokenRequest' не удалось
                match ErrCode {
                    _1 => eprintln!("[ ERROR [: detected when running the request to the Token API"),
                    _2 => eprintln!("[ ERROR [: detected when trying to decode API response"),
                    _3 => eprintln!("[ ERROR [: Unexpected HTTP response code"),
                    _ => eprintln!("[ ERROR [: Unexpected error"),
                };
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "[ ERROR [: Token")))
            }
        };
        // расчет нужного количества токенов для всех запусков
        let RequiredToken = self.GetRuns() * TAXOCLI_LATENCY_TOKEN_COST;
        println!("Required Tokens For This Execution: {}, Available Tokens: {}", RequiredToken, AvailableTokens);
        // если токенов недостаточно возвращает ошибку
        if AvailableTokens < RequiredToken as i32 {
            return Err("[ ERROR [: Insufficient Tokens".into());
        }
        // пауза перед отправкой запроса что бы избежать блокировки из за частоты запросов
        sleep(Duration::from_secs(TAXOCLI_API_THROTTLER_TIME as u64));
        let mut LatencyResult: HashMap<String, f64> = HashMap::new();
        println!("[ INFO [: Sleeping {} Seconds Between Latency Requests", self.GetWaitInterval());
        // цикл для выполнения каждого запроса задержки
        for I in 1..=self.GetRuns() {
            println!("[ INFO [: Request number [{}/{}]", I, self.GetRuns());
            // выполнение запроса не проверку задержки
            let ResponseLatencyCheck = match self.DoPostLatencyCheckRequest(vec![]) {
                Ok(Response) => Response,
                Err(Err) => {
                    eprintln!("[ ERROR [: Doing Latency Check Request: {}", Err);
                    return Err(Box::new(Err))
                }
            };
            // ожидание лучших локаций с минимальной задержкой
            for (Key, Value) in ResponseLatencyCheck.iter() {
                let Latency = Value.get("Latency").unwrap_or(&1000.0);
                let StatusCode = Value.get("StatusCode").unwrap_or(&500.0);
                // если ответ на 200 то присваеваем высокое значение задержкт 1000 мс
                let FinalLatency = if *StatusCode != 200.0 { 1000.0 } else { *Latency };
                *LatencyResult.entry(Key.clone()).or_insert(0.0) += FinalLatency;
            }

            // ожидание перед следующим запросом если он не последний
            if self.GetRuns() > 1 {
                sleep(Duration::from_secs(self.GetWaitInterval()));
            }
        }
        // определение лучших локаций  с минимальнйо задержкой
        let (BestLocations, AvgLatencies) = self.GetMinimumLatencies(LatencyResult);
        let mut OutputList = LatencyCheckerOutputList { Result: Vec::new() };
        // формирование списка результатов включая среднюю задержку для каждой локации
        for I in 0..self.GetOutputLocationsNumber() {
            let AvgLatency  = AvgLatencies[I] / self.GetRuns() as f64;
            let Location = &BestLocations[I];
            OutputList.Result.push(LatencyCheckerOutput {
                AvgLatency,
                Locations: Location.clone()
            });
        }
        // возврашение списка результатов
        Ok(OutputList)
    }
}