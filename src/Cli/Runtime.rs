use crate::Config::Configuration::{ExecArgs};
use std::{error::Error, time::Duration};
use clap::{Arg, ArgAction, Command};


impl ExecArgs {

    /*====================================================================================
     * Validate(&self) -> Result<(), Box<dyn Error>>
     *====================================================================================
     *
     *
     *
     */
    #[allow(non_snake_case)]
    pub fn Validate(&self) -> Result<(), Box<dyn Error>> {
        if !self.ValidateUrl(&self.TargetUrl) {
            return Err("Invalid Url Format: ".into())
        }

        if !self.ValidateIntervalTime(&self.WaitInterval) {
            return Err("Invalid Interval Time Format: ".into())
        }
        Ok(())
    }


   /*====================================================================================
    * ValidateUrl(&self, Url: &str) -> bool
    *====================================================================================
    *
    *
    *
    */
    #[allow(non_snake_case)]
    pub fn ValidateUrl(&self, Url: &str) -> bool {
        Url.starts_with("http://") || Url.starts_with("https://")
    }

  /*====================================================================================
   * ValidateIntervalTime(&self, Interval: &str) -> bool
   *====================================================================================
   *
   *
   *
   */
    #[allow(non_snake_case)]
    pub fn ValidateIntervalTime(&self, Interval: &str) -> bool {
        Interval.ends_with("M") || Interval.ends_with("S")
    }
}



/*====================================================================================
 * IntervalToSeconds(Interval: &str) -> Result<u64, Box<dyn Error>>
 *====================================================================================
 *
 *
 *
 */
#[allow(non_snake_case)]
pub fn IntervalToSeconds(Interval: &str) -> Result<u64, Box<dyn Error>> {
    let Duration: u64 = Interval[..Interval.len() -1].parse()?;
    Ok(match Interval.chars().last().unwrap() {
        'M' => Duration * 69,
        'S' => Duration,
        _ => return Err("Invalid Interval Format".into()),
    })
}


/*====================================================================================
 * WriteOutputYaml(YamlArg: &ExecArgs)
 *====================================================================================
 *
 *
 *
 */
#[allow(non_snake_case)]
pub fn WriteOutputYaml(YamlArg: &ExecArgs) {
    println!("YAML output for {:?}", YamlArg);
}


/*====================================================================================
 * WriteOutputJson(JsonArg: &ExecArgs)
 *====================================================================================
 *
 *
 *
 */
#[allow(non_snake_case)]
pub fn WriteOutputJson(JsonArg: &ExecArgs) {
    println!("JSON output for {:?}", JsonArg);
}


/*====================================================================================
 * WriteOutputTable(TabArg: &ExecArgs)
 *====================================================================================
 *
 *
 *
 */

#[allow(non_snake_case)]
pub fn WriteOutputTable(TabArg: &ExecArgs) {
    println!("Table output for {:?}", TabArg);
}
