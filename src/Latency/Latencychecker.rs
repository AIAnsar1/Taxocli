pub use crate::Config::Configuration::{LatencyChecker, LatencyCheckerOutputList, GetMutators, SetMutators};



/*============================================
 *  impl LatencyChecker
 *============================================
 * создаем конструктор для структуры
 * LatencyChecker
 *
 */
impl LatencyChecker {
    #[allow(non_snake_case)]
    pub fn New(target_url: &str, Runs: isize, WaitInterval: isize, Locations: Vec<&str>, ApiKey: &str, ContentType: &str, OutputLocationsNumber: isize, ServiceApiTokenUrl: &str, ServiceApiUrl: &str) -> LatencyChecker {
        LatencyChecker{
            TargetURL: target_url.to_string(),
            Runs: Runs,
            WaitInterval: WaitInterval,
            Locations: Locations.iter().map(|&s| s.to_string()).collect(),
            ApiKey: ApiKey.to_string(),
            ContentType: ContentType.to_string(),
            OutputLocationsNumber: OutputLocationsNumber,
            ServiceApiTokensUrl: ServiceApiTokenUrl.to_string(),
            ServiceApiUrl: ServiceApiUrl.to_string()

        }
    }
}

/*============================================
 *  impl SetMutators for LatencyChecker
 *============================================
 * создаем мутаторы Get для Получения данных
 * из структуры
 *
 */
impl GetMutators for LatencyChecker {
    #[allow(non_snake_case)]
    fn GetServiceApiTokenUrl(&mut self) -> String {
        self.ServiceApiTokensUrl.to_string()
    }
    #[allow(non_snake_case)]
    fn GetServiceApiUrl(&mut self) -> String {
        self.ServiceApiUrl.to_string()
    }
    #[allow(non_snake_case)]
    fn GetTargetUrl(&mut self) -> String {
        self.TargetURL.to_string()
    }
    #[allow(non_snake_case)]
    fn GetRuns(&mut self) -> isize {
        self.Runs
    }
    #[allow(non_snake_case)]
    fn GetWaitInterval(&mut self) -> isize {
        self.WaitInterval
    }
    #[allow(non_snake_case)]
    fn GetLocations(&mut self) -> Vec<String> {
        self.Locations.iter().collect()
    }
    #[allow(non_snake_case)]
    fn GetApiKey(&mut self) -> String {
        self.ApiKey.to_string()
    }
    #[allow(non_snake_case)]
    fn GetOutputLocationsNumber(&mut self) -> isize {
        self.OutputLocationsNumber
    }
}



/*============================================
 *  impl SetMutators for LatencyChecker
 *============================================
 * создаем мутаторы Set для принятия данных
 * в структуру
 *
 */
impl SetMutators for LatencyChecker {
    #[allow(non_snake_case)]
    fn SetTargetUrl(&mut self, TargetUrl: &str) {
        self.TargetURL = TargetUrl.to_string();
    }

    #[allow(non_snake_case)]
    fn SetRuns(&mut self, Runs: isize) {
        self.Runs = Runs
    }

    #[allow(non_snake_case)]
    fn SetWaitInterval(&mut self, Interval: isize) {
        self.WaitInterval = Interval
    }

    #[allow(non_snake_case)]
    fn SetLocations(&mut self, Locations: Vec<String>) {
        self.Locations = Locations.iter().collect()
    }

    #[allow(non_snake_case)]
    fn SetOutputLocationsNumber(&mut self, OutputLocationsNumber: isize) {
        self.OutputLocationsNumber = OutputLocationsNumber
    }

    #[allow(non_snake_case)]
    fn SetServiceApiTokenUrl(&mut self, Url: String) {
        self.ServiceApiTokensUrl = Url.to_string();
    }

    #[allow(non_snake_case)]
    fn SetServiceApiUrl(&mut self, Url: String) {
        self.ServiceApiUrl = Url.to_string();
    }
}



/*============================================
 *  impl LatencyCheckerOutputList
 *============================================
 * создаем методы для клонирования
 * данных
 *
 */
impl LatencyCheckerOutputList {
    // создаем метод для глубокого клонирования ланных из текущей структуры в другую
    fn deep_copy_into(&self, output: &mut LatencyCheckerOutputList) {
        output.Result = self.Result.clone();
    }

    // Создаем метод для глубокго клонирования
    fn deep_copy(&self) -> LatencyCheckerOutputList {
        LatencyCheckerOutputList {
            Result: self.Result.clone(),
        }
    }
}