use crate::Config::Configuration::{ReptiliaS, TReptilia};
use rand::{Rng, SeedableRng, rngs::StdRng};
use uuid::Uuid;
use fake::{Fake, Faker};


impl ReptiliaS {
    #[allow(non_snake_case)]
    fn New() -> ReptiliaS {
        ReptiliaS {
            FakerGun: Faker,
        }
    }
}



impl TReptilia for ReptiliaS {
    #[allow(non_snake_case)]
    fn ReptiliaBoolean(&mut self) -> bool {
        self.FakerGun.fake::<bool>()
    }
    #[allow(non_snake_case)]
    fn ReptiliaInt(&mut self) -> bool {
        self.FakerGun.fake::<i32>().abs() % 1001
    }
    #[allow(non_snake_case)]
    fn ReptiliaStringMaxLenght(&mut self, Lenght: isize) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFloatBetween(&mut self, MaxDecimals: isize, Min: isize, Max: isize) -> f64 {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaIntBetween(&mut self, Min: isize, Max: isize) -> isize {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaUUID(&mut self) -> Uuid {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCurrentISOTimestamp(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCurrentTimestamp(&mut self) -> i64 {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaGuid(&mut self) -> Uuid {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAddressLongitude(&mut self) -> f64 {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAAddressLatitude(&mut self) -> f64 {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAddressCountry(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAddressStreetAddress(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAddresStreetName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAddressCity(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPersonNameSuffix(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPersonNamePrefix(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPersonFullName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPersonLastName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPersonFirstName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaUserAgent(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLocale(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPassword(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaIP(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaSafeColorHex(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaSafeColorName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDateFuture(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDatePast(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDateRecent(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremWord(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremWords(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremSentence(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPhrase(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremSentences(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremLines(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremParagraph(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremText(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremParagraphs(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLoremSlug(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaNoun(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaVerb(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaIngVerb(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAdjective(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaWord(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaWords(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDepartment(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaProductName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaProductMaterial(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaProductAdjective(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaProduct(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPrice(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFilePath(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaMimeType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDirectoryPath(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileExtension(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCommonFileName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFileType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFileExtension(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFileName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaUrl(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaUsername(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaExampleEmail(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaEmail(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDomainWord(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDomainSuffix(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDomainName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaMonth(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaWeekday(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseColumn(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseCollation(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDatabaseEngine(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhrase(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseAdjective(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseDescriptor(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCatchPhraseNoun(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBsNoun(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBsBuzzWord(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBsAdjective(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBs(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCompanySuffix(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCompanyName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBitcoin(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCurrencySymbol(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCurrencyCode(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCurrencyName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaTransactionType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCreditCardMask(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountName(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountBic(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBankAccountIban(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBankAccount(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDataImageUri(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAvatarImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaImageURL(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAbstractImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAnimalsImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaBusinessImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCatsImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCityImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFoodImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaNightlifeImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFashionImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPeopleImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaNatureImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaSportsImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaTransportImage(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaCountryCode(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPhoneNumber(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaPhoneNumberExt(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaJobArea(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaJobDescriptor(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaJobType(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaJobTitle(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaSemver(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaProtocol(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAbbreviation(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaAlphanumeric(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaIpv6(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDigitNotNull(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaDigit(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaFloat(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaString(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaStringWithLength(&mut self, Length: isize) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaLetter(&mut self) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaIntBetweens(&mut self, Min: isize, Max: isize) -> String {
        todo!()
    }
    #[allow(non_snake_case)]
    fn ReptiliaNewFaker(&mut self) -> String {
        todo!()
    }
}