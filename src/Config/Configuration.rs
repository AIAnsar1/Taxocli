use fake::{Fake, Faker};
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ExecArgs {
    pub(crate) TargetUrl: String,
    pub(crate) NumberOfRuns: usize,
    pub(crate) WaitInterval: String,
    pub(crate) Locations: Vec<String>,
    pub(crate) OutputLocationsNumber: isize,
    pub(crate) OutputFormat: String,
}



#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LatencyChecker {
    pub(crate) TargetURL: String,
    pub(crate) Runs: isize,
    pub(crate) WaitInterval: isize,
    pub(crate) Locations: Vec<String>,
    pub(crate) ApiKey: String,
    pub(crate) ContentType: String,
    pub(crate) OutputLocationsNumber: isize,
    pub(crate) ServiceApiTokensUrl: String,
    pub(crate) ServiceApiUrl: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct LatencyCheckerOutput {
    pub(crate) Locations: String,
    pub(crate) AvgLatency: f64,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct LatencyCheckerOutputList {
    pub(crate) Result: Vec<LatencyCheckerOutput>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TokenApiResponse {
    pub(crate) RequestCount: isize,
    pub(crate) Duration: isize,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LatencyApiRequest {
    pub(crate) TargetUrl: String,
    pub(crate) Locations: Vec<String>,
}


#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ReptiliaS {
    pub(crate) FakerGun: Faker,
}


#[allow(non_snake_case)]
pub trait GetMutators {
    #[allow(non_snake_case)]
    fn GetServiceApiTokenUrl(&mut self) -> String;
    #[allow(non_snake_case)]
    fn GetServiceApiUrl(&mut self) -> String;
    #[allow(non_snake_case)]
    fn GetTargetUrl(&mut self) -> String;
    #[allow(non_snake_case)]
    fn GetRuns(&mut self) -> isize;
    #[allow(non_snake_case)]
    fn GetWaitInterval(&mut self) -> isize;
    #[allow(non_snake_case)]
    fn GetLocations(&mut self) -> Vec<String>;
    #[allow(non_snake_case)]
    fn GetApiKey(&mut self) -> String;
    #[allow(non_snake_case)]
    fn GetOutputLocationsNumber(&mut self) -> isize;
}
#[allow(non_snake_case)]
pub trait SetMutators {
    #[allow(non_snake_case)]
    fn SetTargetUrl(&mut self, TargetUrl: &str);
    #[allow(non_snake_case)]
    fn SetRuns(&mut self, Runs: isize);
    #[allow(non_snake_case)]
    fn SetWaitInterval(&mut self, Interval: isize);
    #[allow(non_snake_case)]
    fn SetLocations(&mut self, Locations: Vec<String>);
    #[allow(non_snake_case)]
    fn SetOutputLocationsNumber(&mut self, OutputLocationsNumber: isize);
    #[allow(non_snake_case)]
    fn SetServiceApiTokenUrl(&mut self, Url: String);
    #[allow(non_snake_case)]
    fn SetServiceApiUrl(&mut self, Url: String);

}


pub trait TReptilia {
    #[allow(non_snake_case)]
    fn ReptiliaBoolean(&mut self) -> bool;
    #[allow(non_snake_case)]
    fn ReptiliaInt(&mut self) -> bool;
    #[allow(non_snake_case)]
    fn ReptiliaStringMaxLenght(&mut self, Lenght: isize) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFloatBetween(&mut self, MaxDecimals: isize, Min: isize, Max: isize) -> f64;
    #[allow(non_snake_case)]
    fn ReptiliaIntBetween(&mut self, Min: isize, Max: isize) -> isize;
    #[allow(non_snake_case)]
    fn ReptiliaUUID(&mut self) -> Uuid;
    #[allow(non_snake_case)]
    fn ReptiliaCurrentISOTimestamp(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCurrentTimestamp(&mut self) -> i64;
    #[allow(non_snake_case)]
    fn ReptiliaGuid(&mut self) -> Uuid;
    #[allow(non_snake_case)]
    fn ReptiliaAddressLongitude(&mut self) -> f64;
    #[allow(non_snake_case)]
    fn ReptiliaAAddressLatitude(&mut self) -> f64;
    #[allow(non_snake_case)]
    fn ReptiliaAddressCountry(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAddressStreetAddress(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAddresStreetName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAddressCity(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPersonNameSuffix(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPersonNamePrefix(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPersonFullName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPersonLastName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPersonFirstName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaUserAgent(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLocale(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPassword(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaIP(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaSafeColorHex(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaSafeColorName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDateFuture(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDatePast(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDateRecent(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremWord(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremWords(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremSentence(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPhrase(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremSentences(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremLines(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremParagraph(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremText(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremParagraphs(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLoremSlug(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaNoun(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaVerb(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaIngVerb(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAdjective(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaWord(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaWords(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDepartment(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaProductName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaProductMaterial(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaProductAdjective(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaProduct(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPrice(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFilePath(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaMimeType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDirectoryPath(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileExtension(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFileType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFileExtension(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFileName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaUrl(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaUsername(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaExampleEmail(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaEmail(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDomainWord(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDomainSuffix(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDomainName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaMonth(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaWeekday(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseColumn(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseCollation(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseEngine(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhrase(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseAdjective(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseDescriptor(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseNoun(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBsNoun(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBsBuzzWord(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBsAdjective(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBs(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCompanySuffix(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCompanyName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBitcoin(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCurrencySymbol(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCurrencyCode(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCurrencyName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaTransactionType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCreditCardMask(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountName(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountBic(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountIban(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBankAccount(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDataImageUri(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAvatarImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaImageURL(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAbstractImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAnimalsImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaBusinessImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCatsImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCityImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFoodImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaNightlifeImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFashionImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPeopleImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaNatureImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaSportsImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaTransportImage(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaCountryCode(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPhoneNumber(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaPhoneNumberExt(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaJobArea(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaJobDescriptor(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaJobType(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaJobTitle(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaSemver(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaProtocol(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAbbreviation(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaAlphanumeric(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaIpv6(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDigitNotNull(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaDigit(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaFloat(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaString(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaStringWithLength(&mut self, Length: isize) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaLetter(&mut self) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaIntBetweens(&mut self, Min: isize, Max: isize) -> String;
    #[allow(non_snake_case)]
    fn ReptiliaNewFaker(&mut self) -> String;



}

pub const TAXOCLI_CONTENT_TYPE_REQUEST: &str = "application/json";
pub const TAXOCLI_TOKEN_API_URL: &str = "https://api.taxocli.cw";
pub const TAXOCLI_LATENCY_API_URL: &str = "https://api.taxocli.cw";
pub const TAXOCLI_LATENCY_TOKEN_COST: isize = 5000;
pub const TAXOCLI_API_THROTTLER_TIME: isize = 1;

pub const JOB_DESCRIPTORS: &[&str] = &[
    "Lead", "Senior", "Direct", "Corporate", "Dynamic", "Future", "Product", "National", "Regional", "District",
    "Central", "Global", "Customer", "Investor", "Dynamic", "International", "Legacy", "Forward", "Internal", "Human",
    "Chief", "Principal"
];

pub const JOB_AREAS: &[&str] = &[
    "Solutions", "Program", "Brand", "Security", "Research", "Marketing", "Directives", "Implementation",
    "Integration", "Functionality", "Response", "Paradigm", "Tactics", "Identity", "Markets", "Group", "Division",
    "Applications", "Optimization", "Operations", "Infrastructure", "Intranet", "Communications", "Web", "Branding",
    "Quality", "Assurance", "Mobility", "Accounts", "Data", "Creative", "Configuration", "Accountability", "Interactions",
    "Factors", "Usability", "Metrics"
];

pub const JOB_TYPES: &[&str] = &[
    "Supervisor", "Associate", "Executive", "Liaison", "Officer", "Manager", "Engineer", "Specialist", "Director",
    "Coordinator", "Administrator", "Architect", "Analyst", "Designer", "Planner", "Orchestrator", "Technician",
    "Developer", "Producer", "Consultant", "Assistant", "Facilitator", "Agent", "Representative", "Strategist"
];

pub const ABBREVIATIONS: &[&str] = &[
    "TCP", "HTTP", "SDD", "RAM", "GB", "CSS", "SSL", "AGP", "SQL", "FTP", "PCI", "AI", "ADP", "RSS", "XML", "EXE", "COM",
    "HDD", "THX", "SMTP", "SMS", "USB", "PNG", "SAS", "IB", "SCSI", "JSON", "XSS", "JBOD"
];

pub const COUNTRY_CODES: &[&str] = &[
    "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX", "AZ", "BA", "BB", "BD",
    "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ", "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA",
    "CC", "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE",
    "DJ", "DK", "DM", "DO", "DZ", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR", "GA",
    "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK",
    "HM", "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP",
    "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC", "LI", "LK", "LR", "LS", "LT",
    "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS",
    "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NU", "NZ",
    "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS",
    "RU", "RW", "SA", "SB", "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST",
    "SV", "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO", "TR", "TT", "TV", "TW",
    "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI", "VN", "VU", "WF", "WS", "YE", "YT", "ZA",
    "ZM", "ZW"
];


pub const PROTOCOLS: &[&str] = &[
    "http", "https"
];

pub const BANK_TRANSACTION_TYPES: &[&str] = &[
    "deposit", "withdrawal", "payment", "invoice"
];

pub const BANK_ACCOUNTS: &[&str] = &[
    "Checking", "Savings", "Money Market", "Investment", "Home Loan", "Credit Card", "Auto Loan", "Personal Loan"
];

pub const BANK_ACCOUNTS_I_BANS: &[&str] = &[
    "HU 26", "AT 18", "MG 25", "CZ 22", "EE 18", "DE 20", "FI 16", "IL 21", "PT 23", "ES 22", "SE 22", "TR 24",
    "UA 27", "MD 22"
];

pub const BANK_ACCOUNTS_BISC: &[&str] = &[
    "BADACVF1", "RZYUAIZ1383", "GJCAMHN1488", "DTVUHMS1", "AAFCSEMM", "LNDXSEGG", "ABVOSEGG", "AFAISESS", "AKRPSESS",
    "AABASESS", "AABASESSTMS", "ALTASESS", "AAFESES2", "ATLPSESS", "ATLPSES2", "AVANSESX", "BFIUSES2", "BKCHSESS",
    "BGABSESG", "BGABSESS", "BGABSESSMUL", "BKABSE22", "BSTPSESS", "FTSBSESS", "BRTESESS", "CITISESX", "BSUISESS",
    "DABASESXGBG", "DABASESX", "DABASESXCLS", "DABASESXCON", "DABASESXSTO", "DNBASESX", "DNBASESXCLS", "MNYXSESS",
    "EPRCSE22", "EQTSSESS", "ERPFSES2", "VPCSSESS", "VPCSSESSCLP", "VPCSSESSVKI", "EKTKSES2", "FORXSESA", "GLSYSES2",
    "GTRESEGG", "GITCSES2", "GLCTSES2", "GFTMSES2", "HMHMSESS", "HMHMSESSGOE", "ICHOSES2", "FTCSSESS", "FTCSSES2",
    "CHASSESS", "JAKMSE22", "KLRNSESS", "KIABSE22", "LAHYSESS", "ELLFSESS", "LANTSESS", "MARGSESS", "MFEXSESS",
    "MFEXSESSGFP", "OMBSSESSNFM", "OMBSSESS", "OMECSESS", "OMECSESSCCP", "OMECSESSNOC", "OMECSESSCOL", "OMECSESSDVP",
    "OMECSESSEXT", "OMECSESSTRE", "OMECSESSTSU", "NOFBSESS", "NDEASEGG", "NDEASESSNCL", "NDEASESSRCC", "NDEASESS",
    "NDEASESSFDS", "NDEASESSTRI", "NDEASESSNBM", "NDEASESSPGI", "NOLSSES2910", "NOLSSES2911", "NOLSSES2913",
    "NOLSSES2G10", "NOLSSES2G11", "NOLSSES2G12", "NOLSSES2G13", "NOLSSES2G14", "NOLSSES2W95", "NOLSSES2W96",
    "NOLSSES2", "NOLSSES2491", "NOLSSES2A96", "NOLSSES2E47", "NOLSSES2E48", "NOLSSES2E49", "NNSESE22", "NOHLSESS",
    "OHMJSESS", "RILUSES2", "SNDVSE22", "SVTRSESS", "AARBDE5W100", "AARBDE5W360", "AARBDE5W860", "AARBDE5WCLE",
    "AARBDE5W500", "AARBDE5W200", "AARBDE5W250", "AARBDE5W550", "AARBDE5W700", "AARBDE5W600", "AARBDE5WDOM", "AARBDE5W",
    "IMMODE5M", "ABCADEFFKTO", "ABCADEFF", "WWBADE3ADOT", "WWBADE3A", "ABKBDEBB", "ABKSDEFF", "FTSBDEFAMYO",
    "FTSBDEFAPRO", "FTSBDEFASFI", "FTSBDEFA", "AWSGDE3S", "WUERDE66", "ABOCDEFF", "AGBMDEMMTGT", "AGBMDEMME22",
    "AGBMDEMME23", "AGBMDEMM", "EADSDEMMPAY", "EADSDEMM", "AUSKDEFF", "AKBKDEFF", "AKFBDE33", "AGIDDEFB", "AGIDDEFFHOF",
    "AGIDDEFF", "AGIDDEFBSGF", "AGIDDEFFPAR", "AGIDDEFBSSL", "AGIDDEFFIAM", "AGIDDEFFPA1", "AGIDDEFFFOA", "AGIDDEFFHO1",
    "AGIDDEFFHO3", "AGIDDEFFINV", "ALLVDESL", "AZSEDEMM", "SOGEFRPPAGY", "SOGEFRPPAOA", "SOGEFRPPOBA", "SOGEFRPPANB",
    "SOGEFRPPRRC", "SOGEFRPPABO", "SOGEFRPPBML", "SOGEFRPPAJC", "SOGEFRPPCTE", "SOGEFRPPBXC", "SOGEFRPPUCR",
    "SOGEFRPPBFD", "SOGEFRPPAED", "SOGEFRPPWLF", "SOGEFRPPAZF", "SOGEFRPPAUG", "SOGEFRPPGSG", "SOGEFRPPALG",
    "SOGEFRPPAGK", "SOGEFRPPBMA", "SOGEFRPPAKT", "SOGEFRPPFMA", "SOGEFRPPAMM", "SOGEFRPPLPC", "SOGEFRPPHPO",
    "SOGEFRPPKPA", "SOGEFRPPJLH", "SOGEFRPPRAS", "SOGEFRPPBKR", "SOGEFRPPRIG", "SOGEFRPPGSM", "SOGEFRPPAFS",
    "SOGEFRPPAPS", "SOGEFRPPAIS", "SOGEFRPPAXT", "SOGEFRPPPVL", "SOGEFRPPATV", "SOGEFRPPTVI", "SOGEFRPPBLG",
    "SOGEFRPPAAA", "SOGEFRPPACF", "SOGEFRPPSFX", "SOGEFRPPSGA", "SOGEFRPPCBV", "SOGEFRPPSGI", "SOGEFRPPBTC",
    "SOGEFRPPBTL", "SOGEFRPPFIP", "SOGEFRPPSGO", "SOGEFRPPLMT", "DEUTTRIS", "BKTRTRIS", "BKTRTRISCUS", "BKTRTRISGFX",
    "DYAKTRIS", "DSGHTRIS", "ECTVTRIS", "EKSFTRI2", "FBHLTRIS", "GLOHTRIS", "GOGYTRIS", "GSDBTRIS", "HABBTRIS",
    "HSBCTRIX002", "HSBCTRIX113", "HSBCTRIX933", "HSBCTRIX956", "HSBCTRIX001", "HSBCTRIXHYM", "HSBCTRIX", "HSBYTRIS",
    "ICBKTRISADN", "ICBKTRISANK", "ICBKTRISNEC", "ICBKTRISOST", "ICBKTRISATK", "ICBKTRISANT", "ICBKTRISAYD",
    "ICBKTRISBAL", "ICBKTRISBUR", "ICBKTRISDNZ", "ICBKTRISESK", "ICBKTRISISK", "ICBKTRISAVC", "ICBKTRISBKY",
    "ICBKTRISBYP", "ICBKTRISBSK", "ICBKTRISDLY", "ICBKTRISMTP", "ICBKTRISETL", "ICBKTRISGUN", "ICBKTRISIKT",
    "ICBKTRISTZL", "ICBKTRISKDK", "ICBKTRISKZY", "ICBKTRISLVT", "ICBKTRISMER", "ICBKTRISMAL", "ICBKTRISBAK",
    "ICBKTRISMOD"
];

pub const CURRENCY_NAMES: &[&str] = &[
    "UAE Dirham", "Afghani", "Lek", "Armenian Dram", "Netherlands Antillian Guilder", "Kwanza", "Argentine Peso",
    "Australian Dollar", "Aruban Guilder", "Azerbaijanian Manat", "Convertible Marks", "Barbados Dollar", "Taka",
    "Bulgarian Lev", "Bahraini Dinar", "Burundi Franc", "Bermudian Dollar (customarily known as Bermuda Dollar)",
    "Brunei Dollar", "Boliviano boliviano", "Brazilian Real", "Bahamian Dollar", "Pula", "Belarussian Ruble",
    "Belize Dollar", "Canadian Dollar", "Congolese Franc", "Swiss Franc", "Chilean Peso", "Yuan Renminbi",
    "Colombian Peso", "Costa Rican Colon", "Cuban Peso", "Cuban Peso Convertible", "Cape Verde Escudo", "Czech Koruna",
    "Djibouti Franc", "Danish Krone", "Dominican Peso", "Algerian Dinar", "Kroon", "Egyptian Pound", "Nakfa",
    "Ethiopian Birr", "Euro", "Fiji Dollar", "Falkland Islands Pound", "Pound Sterling", "Lari", "Cedi",
    "Gibraltar Pound", "Dalasi", "Guinea Franc", "Quetzal", "Guyana Dollar", "Hong Kong Dollar", "Lempira",
    "Croatian Kuna", "Gourde", "Forint", "Rupiah", "New Israeli Sheqel", "Bhutanese Ngultrum", "Indian Rupee",
    "Iraqi Dinar", "Iranian Rial", "Iceland Krona", "Jamaican Dollar", "Jordanian Dinar", "Yen", "Kenyan Shilling", "Som",
    "Riel", "Comoro Franc", "North Korean Won", "Won", "Kuwaiti Dinar", "Cayman Islands Dollar", "Tenge", "Kip",
    "Lebanese Pound", "Sri Lanka Rupee", "Liberian Dollar", "Lithuanian Litas", "Latvian Lats", "Libyan Dinar",
    "Moroccan Dirham", "Moldovan Leu", "Malagasy Ariary", "Denar", "Kyat", "Tugrik", "Pataca", "Ouguiya",
    "Mauritius Rupee", "Rufiyaa", "Kwacha", "Mexican Peso", "Malaysian Ringgit", "Tunisian Dinar", "Zambian Kwacha",
    "Metical", "Naira", "Cordoba Oro", "Norwegian Krone", "Nepalese Rupee", "New Zealand Dollar", "Rial Omani",
    "Balboa", "Nuevo Sol", "Kina", "Philippine Peso", "Pakistan Rupee", "Zloty", "Guarani", "Qatari Rial", "New Leu",
    "Serbian Dinar", "Russian Ruble", "Rwanda Franc", "Saudi Riyal", "Solomon Islands Dollar", "Seychelles Rupee",
    "Sudanese Pound", "Swedish Krona", "Singapore Dollar", "Saint Helena Pound", "Leone", "Somali Shilling",
    "Surinam Dollar", "Dobra", "El Salvador Colon", "Syrian Pound", "Lilangeni", "Baht", "Somoni", "Manat",
    "Paanga", "Turkish Lira", "Trinidad and Tobago Dollar", "New Taiwan Dollar", "Tanzanian Shilling", "Hryvnia",
    "Uganda Shilling", "US Dollar", "Peso Uruguayo", "Uzbekistan Sum", "Bolivar Fuerte", "Dong", "Vatu", "Tala",
    "CFA Franc BEAC", "Silver", "Gold", "Bond Markets Units European Composite Unit (EURCO)",
    "European Monetary Unit (E.M.U.-6)", "European Unit of Account 9(E.U.A.-9)", "European Unit of Account 17(E.U.A.-17)",
    "East Caribbean Dollar", "SDR", "UIC-Franc", "CFA Franc BCEAO", "Palladium", "CFP Franc", "Platinum",
    "Codes specifically reserved for testing purposes", "Yemeni Rial", "Rand", "Lesotho Loti", "Namibia Dollar",
    "Zimbabwe Dollar"
];

pub const CURRENCY_CODES: &[&str] = &[
    "AED", "AFN", "ALL", "AMD", "ANG", "AOA", "ARS", "AUD", "AWG", "AZN", "BAM", "BBD", "BDT", "BGN", "BHD", "BIF", "BMD",
    "BND", "BOB", "BRL", "BSD", "BWP", "BYR", "BZD", "CAD", "CDF", "CHF", "CLP", "CNY", "COP", "CRC", "CUP", "CUC", "CVE",
    "CZK", "DJF", "DKK", "DOP", "DZD", "EEK", "EGP", "ERN", "ETB", "EUR", "FJD", "FKP", "GBP", "GEL", "GHS", "GIP", "GMD",
    "GNF", "GTQ", "GYD", "HKD", "HNL", "HRK", "HTG", "HUF", "IDR", "ILS", "BTN", "INR", "IQD", "IRR", "ISK", "JMD", "JOD",
    "JPY", "KES", "KGS", "KHR", "KMF", "KPW", "KRW", "KWD", "KYD", "KZT", "LAK", "LBP", "LKR", "LRD", "LTL", "LVL", "LYD",
    "MAD", "MDL", "MGA", "MKD", "MMK", "MNT", "MOP", "MRO", "MUR", "MVR", "MWK", "MXN", "MYR", "MZN", "NGN", "NIO", "NOK",
    "NPR", "NZD", "OMR", "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG", "QAR", "RON", "RSD", "RUB", "RWF", "SAR", "SBD",
    "SCR", "SDG", "SEK", "SGD", "SHP", "SLL", "SOS", "SRD", "STN", "SVC", "SYP", "SZL", "THB", "TJS", "TMT", "TND", "TOP",
    "TRY", "TTD", "TWD", "TZS", "UAH", "UGX", "USD", "UYU", "UZS", "VEF", "VND", "VUV", "WST", "XAF", "XAG", "XAU", "XBA",
    "XBB", "XBC", "XBD", "XCD", "XDR", "XFU", "XOF", "XPD", "XPF", "XPT", "XTS", "YER", "ZAR", "LSL", "NAD", "ZMK", "ZWL",
];

pub const CURRENCY_SYMBOLS: &[&str] = &[
    "؋", "Lek", "ƒ", "$", "$", "ƒ", "ман", "KM", "$", "лв", "$", "$", "Bs", "R$", "$", "P", "p.", "BZ$", "$", "CHF", "$",
    "¥", "$", "₡", "₱", "$", "Kč", "kr", "RD$", "£", "€", "$", "£", "£", "£", "Q", "$", "$", "L", "kn", "Ft", "Rp", "₪",
    "Nu", "₹", "﷼", "kr", "J$", "¥", "лв", "៛", "₩", "₩", "$", "лв", "₭", "£", "₨", "$", "Lt", "Ls", "ден", "₮", "₨", "$",
    "RM", "MT", "₦", "C$", "kr", "₨", "$", "﷼", "B/.", "S/.", "Php", "₨", "zł", "Gs", "﷼", "lei", "Дин.", "﷼", "$",
    "₨", "kr", "$", "£", "S", "$", "Db", "₡", "£", "฿", "₺", "TT$", "NT$", "₴", "$", "$U", "лв", "Bs", "₫", "$", "﷼", "R",
    "N$"
];

pub const COMPANY_SUFFIXES: &[&str] = &[
    "AG", "GmbH", "und Söhne", "und Partner", "& Co.", "Gruppe", "LLC", "Inc."
];

pub const BUSSINES_VERBS: &[&str] = &[
    "implement", "utilize", "integrate", "streamline", "optimize", "evolve", "transform", "embrace", "enable",
    "orchestrate", "leverage", "reinvent", "aggregate", "architect", "enhance", "incentivize", "morph", "empower",
    "envisioneer", "monetize", "harness", "facilitate", "seize", "disintermediate", "synergize", "strategize", "deploy",
    "brand", "grow", "target", "syndicate", "synthesize", "deliver", "mesh", "incubate", "engage", "maximize",
    "benchmark", "expedite", "reintermediate", "whiteboard", "visualize", "repurpose", "innovate", "scale", "unleash",
    "drive", "extend", "engineer", "revolutionize", "generate", "exploit", "transition", "e-enable", "iterate",
    "cultivate", "matrix", "productize", "redefine", "recontextualize",
];

pub const BUSSINES_ADJECTIVES: &[&str] = &[
    "clicks-and-mortar", "value-added", "vertical", "proactive", "robust", "revolutionary", "scalable", "leading-edge",
    "innovative", "intuitive", "strategic", "e-business", "mission-critical", "sticky", "one-to-one", "24/7",
    "end-to-end", "global", "B2B", "B2C", "granular", "frictionless", "virtual", "viral", "dynamic", "24/365",
    "best-of-breed", "killer", "magnetic", "bleeding-edge", "web-enabled", "interactive", "dot-com", "sexy", "back-end",
    "real-time", "efficient", "front-end", "distributed", "seamless", "extensible", "turn-key", "world-class",
    "open-source", "cross-platform", "cross-media", "synergistic", "bricks-and-clicks", "out-of-the-box", "enterprise",
    "integrated", "impactful", "wireless", "transparent", "next-generation", "cutting-edge", "user-centric", "visionary",
    "customized", "ubiquitous", "plug-and-play", "collaborative", "compelling", "holistic", "rich",
];

pub const BUSSINES_NOUNS: &[&str] = &[
    "synergies", "web-readiness", "paradigms", "markets", "partnerships", "infrastructures", "platforms", "initiatives",
    "channels", "eyeballs", "communities", "ROI", "solutions", "e-tailers", "e-services", "action-items", "portals",
    "niches", "technologies", "content", "vortals", "supply-chains", "convergence", "relationships", "architectures",
    "interfaces", "e-markets", "e-commerce", "systems", "bandwidth", "infomediaries", "models", "mindshare",
    "deliverables", "users", "schemas", "networks", "applications", "metrics", "e-business", "functionalities",
    "experiences", "web services", "methodologies", "blockchains",
];

pub const COMPANY_ADJECTIVES: &[&str] = &[
    "Adaptive", "Advanced", "Ameliorated", "Assimilated", "Automated", "Balanced", "Business-focused", "Centralized",
    "Cloned", "Compatible", "Configurable", "Cross-group", "Cross-platform", "Customer-focused", "Customizable",
    "Decentralized", "De-engineered", "Devolved", "Digitized", "Distributed", "Diverse", "Down-sized", "Enhanced",
    "Enterprise-wide", "Ergonomic", "Exclusive", "Expanded", "Extended", "Face to face", "Focused", "Front-line",
    "Fully-configurable", "Function-based", "Fundamental", "Future-proofed", "Grass-roots", "Horizontal", "Implemented",
    "Innovative", "Integrated", "Intuitive", "Inverse", "Managed", "Mandatory", "Monitored", "Multi-channeled",
    "Multi-lateral", "Multi-layered", "Multi-tiered", "Networked", "Object-based", "Open-architected", "Open-source",
    "Operative", "Optimized", "Optional", "Organic", "Organized", "Persevering", "Persistent", "Phased", "Polarized",
    "Pre-emptive", "Proactive", "Profit-focused", "Profound", "Programmable", "Progressive", "Public-key",
    "Quality-focused", "Reactive", "Realigned", "Re-contextualized", "Re-engineered", "Reduced", "Reverse-engineered",
    "Right-sized", "Robust", "Seamless", "Secured", "Self-enabling", "Sharable", "Stand-alone", "Streamlined",
    "Switchable", "Synchronized", "Synergistic", "Synergized", "Team-oriented", "Total", "Triple-buffered", "Universal",
    "Up-sized", "Upgradable", "User-centric", "User-friendly", "Versatile", "Virtual", "Visionary", "Vision-oriented",
];

pub const COMPANY_DESCRIPTORS: &[&str] = &[
    "24 hour", "24/7", "3rd generation", "4th generation", "5th generation", "6th generation", "actuating", "analyzing",
    "asymmetric", "asynchronous", "attitude-oriented", "background", "bandwidth-monitored", "bi-directional",
    "bifurcated", "bottom-line", "clear-thinking", "client-driven", "client-server", "coherent", "cohesive", "composite",
    "context-sensitive", "contextually-based", "content-based", "dedicated", "demand-driven", "didactic", "directional",
    "discrete", "disintermediate", "dynamic", "eco-centric", "empowering", "encompassing", "even-keeled", "executive",
    "explicit", "exuding", "fault-tolerant", "foreground", "fresh-thinking", "full-range", "global", "grid-enabled",
    "heuristic", "high-level", "holistic", "homogeneous", "human-resource", "hybrid", "impactful", "incremental",
    "intangible", "interactive", "intermediate", "leading edge", "local", "logistical", "maximized", "methodical",
    "mission-critical", "mobile", "modular", "motivating", "multimedia", "multi-state", "multi-tasking", "national",
    "needs-based", "neutral", "next generation", "non-volatile", "object-oriented", "optimal", "optimizing", "radical",
    "real-time", "reciprocal", "regional", "responsive", "scalable", "secondary", "solution-oriented", "stable", "static",
    "systematic", "systemic", "system-worthy", "tangible", "tertiary", "transitional", "uniform", "upward-trending",
    "user-facing", "value-added", "web-enabled", "well-modulated", "zero administration", "zero defect", "zero tolerance",
];

pub const COMPANY_NOUNS: &[&str] = &[
    "ability", "access", "adapter", "algorithm", "alliance", "analyzer", "application", "approach", "architecture",
    "archive", "artificial intelligence", "array", "attitude", "benchmark", "budgetary management", "capability",
    "capacity", "challenge", "circuit", "collaboration", "complexity", "concept", "conglomeration", "contingency", "core",
    "customer loyalty", "database", "data-warehouse", "definition", "emulation", "encoding", "encryption", "extranet",
    "firmware", "flexibility", "focus group", "forecast", "frame", "framework", "function", "functionalities",
    "Graphic Interface", "groupware", "Graphical User Interface", "hardware", "help-desk", "hierarchy", "hub",
    "implementation", "info-mediaries", "infrastructure", "initiative", "installation", "instruction set", "interface",
    "internet solution", "intranet", "knowledge user", "knowledge base", "local area network", "leverage", "matrices",
    "matrix", "methodology", "middleware", "migration", "model", "moderator", "monitoring", "moratorium", "neural-net",
    "open architecture", "open system", "orchestration", "paradigm", "parallelism", "policy", "portal",
    "pricing structure", "process improvement", "product", "productivity", "project", "projection", "protocol",
    "secured line", "service-desk", "software", "solution", "standardization", "strategy", "structure", "success",
    "superstructure", "support", "synergy", "system engine", "task-force", "throughput", "time-frame", "toolset",
    "utilization", "website", "workforce",
];


pub const DATABASE_COLUMNS: &[&str] = &[
    "id", "title", "name", "email", "phone", "token", "group", "category", "password", "comment", "avatar", "status",
    "createdAt", "updatedAt",
];

pub const DATABASE_TYPES: &[&str] = &[
    "int", "varchar", "text", "date", "datetime", "tinyint", "time", "timestamp", "smallint", "mediumint", "bigint",
    "decimal", "float", "double", "real", "bit", "boolean", "serial", "blob", "binary", "enum", "set", "geometry",
    "point",
];

pub const DATABASE_COLLECTIONS: &[&str] = &[
    "utf8_unicode_ci", "utf8_general_ci", "utf8_bin", "ascii_bin", "ascii_general_ci", "cp1250_bin", "cp1250_general_ci"
];

pub const DATABASE_ENGINES: &[&str] = &[
    "InnoDB", "MyISAM", "MEMORY", "CSV", "BLACKHOLE", "ARCHIVE"
];


pub const WEEK_DAYS: &[&str] = &[
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
];

pub const MONTHS: &[&str] = &[
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

pub const DOMAIN_SUFFIXES: &[&str] = &[
    "ac.uk", "biz", "co", "co.uk", "com", "cymru", "gov.uk", "info", "london", "ltd.uk", "me.uk", "name", "nhs.uk",
    "org.uk", "plc.uk", "sch.uk", "scot", "uk", "wales", "com.tr",
];

pub const FIRSTNAMES: &[&str] = &[
    "William", "Jack", "Oliver", "Joshua", "Thomas", "Lachlan", "Cooper", "Noah", "Ethan", "Lucas", "James", "Samuel",
    "Jacob", "Liam", "Alexander", "Benjamin", "Max", "Isaac", "Daniel", "Riley", "Ryan", "Charlie", "Tyler", "Jake",
    "Matthew", "Xavier", "Harry", "Jayden", "Nicholas", "Harrison", "Levi", "Luke", "Adam", "Henry", "Aiden", "Dylan",
    "Oscar", "Michael", "Jackson", "Logan", "Joseph", "Blake", "Nathan", "Connor", "Elijah", "Nate", "Archie", "Bailey",
    "Marcus", "Cameron", "Jordan", "Zachary", "Caleb", "Hunter", "Ashton", "Toby", "Aidan", "Hayden", "Mason", "Hamish",
    "Edward", "Angus", "Eli", "Sebastian", "Christian", "Patrick", "Andrew", "Anthony", "Luca", "Kai", "Beau", "Alex",
    "George", "Callum", "Finn", "Zac", "Mitchell", "Jett", "Jesse", "Gabriel", "Leo", "Declan", "Charles", "Jasper",
    "Jonathan", "Aaron", "Hugo", "David", "Christopher", "Chase", "Owen", "Justin", "Ali", "Darcy", "Lincoln", "Cody",
    "Phoenix", "Sam", "John", "Joel", "Isabella", "Ruby", "Chloe", "Olivia", "Charlotte", "Mia", "Lily", "Emily",
    "Ella", "Sienna", "Sophie", "Amelia", "Grace", "Ava", "Zoe", "Emma", "Sophia", "Matilda", "Hannah", "Jessica",
    "Lucy", "Georgia", "Sarah", "Abigail", "Zara", "Eva", "Scarlett", "Jasmine", "Chelsea", "Lilly", "Ivy", "Isla",
    "Evie", "Isabelle", "Maddison", "Layla", "Summer", "Annabelle", "Alexis", "Elizabeth", "Bella", "Holly", "Lara",
    "Madison", "Alyssa", "Maya", "Tahlia", "Claire", "Hayley", "Imogen", "Jade", "Ellie", "Sofia", "Addison", "Kiara",
    "Molly", "Phoebe", "Alice", "Savannah", "Gabriella", "Kayla", "Mikayla", "Abbey", "Eliza", "Willow", "Alexandra",
    "Poppy", "Samantha", "Stella", "Amy", "Amelie", "Anna", "Piper", "Gemma", "Isabel", "Victoria", "Stephanie",
    "Caitlin", "Heidi", "Paige", "Rose", "Amber", "Audrey", "Claudia", "Taylor", "Madeline", "Angelina", "Natalie",
    "Charli", "Lauren", "Ashley", "Violet", "Mackenzie", "Abby", "Skye", "Lillian", "Alana", "Lola", "Leah", "Eve",
];

pub const LASTNAMES: &[&str] = &[
    "Smith", "Jones", "Williams", "Brown", "Wilson", "Taylor", "Johnson", "White", "Martin", "Anderson", "Thompson",
    "Nguyen", "Thomas", "Walker", "Harris", "Lee", "Ryan", "Robinson", "Kelly", "King", "Davis", "Wright", "Evans",
    "Roberts", "Green", "Hall", "Wood", "Jackson", "Clarke", "Patel", "Khan", "Lewis", "James", "Phillips", "Mason",
    "Mitchell", "Rose", "Davies", "Rodriguez", "Cox", "Alexander", "Garden", "Campbell", "Johnston", "Moore", "Smyth",
    "Oneill", "Doherty", "Stewart", "Quinn", "Murphy", "Graham", "Mclaughlin", "Hamilton", "Murray", "Hughes",
    "Robertson", "Thomson", "Scott", "Macdonald", "Reid", "Clark", "Ross", "Young", "Watson", "Paterson", "Morrison",
    "Morgan", "Griffiths", "Edwards", "Rees", "Jenkins", "Owen", "Price", "Moss", "Richards", "Abbott", "Adams",
    "Armstrong", "Bahringer", "Bailey", "Barrows", "Bartell", "Bartoletti", "Barton", "Bauch", "Baumbach", "Bayer",
    "Beahan", "Beatty", "Becker", "Beier", "Berge", "Bergstrom", "Bode", "Bogan", "Borer", "Bosco", "Botsford", "Boyer",
    "Boyle", "Braun", "Bruen", "Carroll", "Carter", "Cartwright", "Casper", "Cassin", "Champlin", "Christiansen",
    "Cole", "Collier", "Collins", "Connelly", "Conroy", "Corkery", "Cormier", "Corwin", "Cronin", "Crooks", "Cruickshank",
    "Cummings", "Damore", "Daniel", "Dare", "Daugherty", "Dickens", "Dickinson", "Dietrich", "Donnelly", "Dooley",
    "Douglas", "Doyle", "Durgan", "Ebert", "Emard", "Emmerich", "Erdman", "Ernser", "Fadel", "Fahey", "Farrell", "Fay",
    "Feeney", "Feil", "Ferry", "Fisher", "Flatley", "Gibson", "Gleason", "Glover", "Goldner", "Goodwin", "Grady", "Grant",
    "Greenfelder", "Greenholt", "Grimes", "Gutmann", "Hackett", "Hahn", "Haley", "Hammes", "Hand", "Hane", "Hansen",
    "Harber", "Hartmann", "Harvey", "Hayes", "Heaney", "Heathcote", "Heller", "Hermann", "Hermiston", "Hessel",
    "Hettinger", "Hickle", "Hill", "Hills", "Hoppe", "Howe", "Howell", "Hudson", "Huel", "Hyatt", "Jacobi", "Jacobs",
    "Jacobson", "Jerde", "Johns", "Keeling", "Kemmer", "Kessler", "Kiehn", "Kirlin", "Klein", "Koch", "Koelpin",
    "Kohler", "Koss", "Kovacek", "Kreiger", "Kris", "Kuhlman", "Kuhn", "Kulas", "Kunde", "Kutch", "Lakin", "Lang",
    "Langworth", "Larkin", "Larson", "Leannon", "Leffler", "Little", "Lockman", "Lowe", "Lynch", "Mann", "Marks",
    "Marvin", "Mayer", "Mccullough", "Mcdermott", "Mckenzie", "Miller", "Mills", "Monahan", "Morissette", "Mueller",
    "Muller", "Nader", "Nicolas", "Nolan", "Oconnell", "Oconner", "Ohara", "Okeefe", "Olson", "Oreilly", "Parisian",
    "Parker", "Quigley", "Reilly", "Reynolds", "Rice", "Ritchie", "Rohan", "Rolfson", "Rowe", "Russel", "Rutherford",
    "Sanford", "Sauer", "Schmidt", "Schmitt", "Schneider", "Schroeder", "Schultz", "Shields", "Smitham", "Spencer",
    "Stanton", "Stark", "Stokes", "Swift", "Tillman", "Towne", "Tremblay", "Tromp", "Turcotte", "Turner", "Walsh",
    "Walter", "Ward", "Waters", "Weber", "Welch", "West", "Wilderman", "Wilkinson", "Williamson", "Windler", "Wolf",
];


pub const EMAILS: &[&str] = &[
    "gmail.com", "yahoo.com", "hotmail.com", "protonmail.com", "outlook.com", "hey.com"
];

pub const EXAMPLE_EMAILS: &[&str] = &[
    "example.org", "example.com", "example.net"
];

pub const DIRECTORY_PATHS: &[&str] = &[
    "/Applications", "/bin", "/boot", "/boot/defaults", "/dev", "/etc", "/etc/defaults", "/etc/mail", "/etc/namedb",
    "/etc/periodic", "/etc/ppp", "/home", "/home/user", "/home/user/dir", "/lib", "/Library", "/lost+found", "/media",
    "/mnt", "/net", "/Network", "/opt", "/opt/bin", "/opt/include", "/opt/lib", "/opt/sbin", "/opt/share", "/private",
    "/private/tmp", "/private/var", "/proc", "/rescue", "/root", "/sbin", "/selinux", "/srv", "/sys", "/System", "/tmp",
    "/Users", "/usr", "/usr/X11R6", "/usr/bin", "/usr/include", "/usr/lib", "/usr/libdata", "/usr/libexec",
    "/usr/local/src", "/usr/obj", "/usr/ports", "/usr/sbin", "/usr/share", "/usr/src", "/var", "/var/log", "/var/mail",
    "/var/spool", "/var/tmp", "/var/yp", "/usr/local/bin",
];


pub const FILE_EXTENSIONS: &[&str] = &[
    "png", "gif", "doc", "docx", "pdf", "xls", "ppt", "jpg", "mp4", "mov", "wav", "html", "json", "xml", "tif", "tsv",
    "csv", "gdoc", "lzh", "wsdl", "html", "war", "book", "fsc",
];


pub const COMMON_FILE_EXTENSIONS: &[&str] = &[
    "pdf", "mpeg", "wav", "png", "jpeg", "gif", "mp4", "html", "m3a",
];

pub const FILE_TYPES: &[&str] = &[
    "video", "audio", "image", "text", "application", "multipart", "model"
];


pub const COMMON_FILE_TYPES: &[&str] = &[
    "video", "audio", "image", "text", "application"
];

pub const COMMON_MIME_TYPES: &[&str] = &[
    "application/pdf", "audio/mpeg", "audio/wav", "image/png", "image/jpeg", "image/gif", "video/mp4", "video/mpeg",
    "text/html",
];

pub const PRODUCTS: &[&str] = &[
    "Chair", "Car", "Computer", "Keyboard", "Mouse", "Bike", "Ball", "Gloves", "Pants", "Shirt", "Table", "Shoes", "Hat",
    "Towels", "Soap", "Tuna", "Chicken", "Fish", "Cheese", "Bacon", "Pizza", "Salad", "Sausages", "Chips",
];


pub const PRODUCT_MATERIALS: &[&str] = &[
    "Steel", "Bronze", "Wooden", "Concrete", "Plastic", "Cotton", "Granite", "Rubber", "Metal", "Soft", "Fresh", "Frozen",
];


pub const PRODUCT_ADJECTIVES: &[&str] = &[
    "Small", "Ergonomic", "Electronic", "Rustic", "Intelligent", "Gorgeous", "Incredible", "Elegant", "Fantastic",
    "Practical", "Modern", "Recycled", "Sleek", "Bespoke", "Awesome", "Generic", "Handcrafted", "Handmade", "Oriental",
    "Licensed", "Luxurious", "Refined", "Unbranded", "Tasty",
];


pub const STORE_DEPARTAMENTS: &[&str] = &[
    "Books", "Movies", "Music", "Games", "Electronics", "Computers", "Home", "Garden", "Tools", "Grocery", "Health",
    "Beauty", "Toys", "Kids", "Baby", "Clothing", "Shoes", "Jewelery", "Sports", "Outdoors", "Automotive", "Industrial",
];


pub const NOUNS: &[&str] = &[
    "driver", "protocol", "bandwidth", "panel", "microchip", "program", "port", "card", "array", "interface", "system",
    "sensor", "firewall", "hard drive", "pixel", "alarm", "feed", "monitor", "application", "transmitter", "bus",
    "circuit", "capacitor", "matrix",
];


pub const VERBS: &[&str] = &[
    "back up", "bypass", "hack", "override", "compress", "copy", "navigate", "index", "connect", "generate", "quantify",
    "calculate", "synthesize", "input", "transmit", "program", "reboot", "parse",
];


pub const ING_VERBS: &[&str] = &[
    "backing up", "bypassing", "hacking", "overriding", "compressing", "copying", "navigating", "indexing", "connecting",
    "generating", "quantifying", "calculating", "synthesizing", "transmitting", "programming", "parsing",
];


pub const ADJECTIVES: &[&str] = &[
    "auxiliary", "primary", "back-end", "digital", "open-source", "virtual", "cross-platform", "redundant", "online",
    "haptic", "multi-byte", "bluetooth", "wireless", "1080p", "neural", "optical", "solid state", "mobile",
];


pub const LOREM_WORDS: &[&str] = &[
    "alias", "consequatur", "aut", "perferendis", "sit", "voluptatem", "accusantium", "doloremque", "aperiam", "eaque",
    "ipsa", "quae", "ab", "illo", "inventore", "veritatis", "et", "quasi", "architecto", "beatae", "vitae", "dicta",
    "sunt", "explicabo", "aspernatur", "aut", "odit", "aut", "fugit", "sed", "quia", "consequuntur", "magni", "dolores",
    "eos", "qui", "ratione", "voluptatem", "sequi", "nesciunt", "neque", "dolorem", "ipsum", "quia", "dolor", "sit",
    "amet", "consectetur", "adipisci", "velit", "sed", "quia", "non", "numquam", "eius", "modi", "tempora", "incidunt",
    "ut", "labore", "et", "dolore", "magnam", "aliquam", "quaerat", "voluptatem", "ut", "enim", "ad", "minima", "veniam",
    "quis", "nostrum", "exercitationem", "ullam", "corporis", "nemo", "enim", "ipsam", "voluptatem", "quia", "voluptas",
    "sit", "suscipit", "laboriosam", "nisi", "ut", "aliquid", "ex", "ea", "commodi", "consequatur", "quis", "autem",
    "vel", "eum", "iure", "reprehenderit", "qui", "in", "ea", "voluptate", "velit", "esse", "quam", "nihil", "molestiae",
    "et", "iusto", "odio", "dignissimos", "ducimus", "qui", "blanditiis", "praesentium", "laudantium", "totam", "rem",
    "voluptatum", "deleniti", "atque", "corrupti", "quos", "dolores", "et", "quas", "molestias", "excepturi", "sint",
    "occaecati", "cupiditate", "non", "provident", "sed", "ut", "perspiciatis", "unde", "omnis", "iste", "natus",
    "error", "similique", "sunt", "culpa", "qui", "officia", "deserunt", "mollitia", "animi", "id", "est", "tenetur",
    "laborum", "et", "dolorum", "fuga", "et", "harum", "quidem", "rerum", "facilis", "est", "et", "expedita", "et", "in",
    "distinctio", "nam", "libero", "tempore", "cum", "soluta", "nobis", "est", "eligendi", "optio", "cumque", "nihil",
    "impedit", "quo", "porro", "quisquam", "est", "qui", "minus", "id", "quod", "maxime", "placeat", "facere", "possimus",
    "omnis", "voluptas", "assumenda", "est", "omnis", "dolor", "repellendus", "temporibus", "autem", "quibusdam", "et",
    "aut", "consequatur", "vel", "illum", "qui", "dolorem", "eum", "fugiat", "quo", "voluptas", "nulla", "pariatur", "at",
    "vero", "eos", "et", "accusamus", "officiis", "debitis", "aut", "rerum", "necessitatibus", "saepe", "eveniet", "ut",
    "voluptates", "repudiandae", "sint", "et", "molestiae", "non", "recusandae", "itaque", "earum", "rerum", "hic", "ut",
    "a", "sapiente", "delectus", "aut", "reiciendis", "voluptatibus", "maiores", "doloribus", "asperiores", "repellat",
];