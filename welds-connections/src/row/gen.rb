
head = %Q|
use super::*;

#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use sqlx::Row as SqlxRow;
|

def blocky(cfg, wheres)
  %Q|

#{cfg}
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: #{wheres}
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: #{wheres}
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
        }
    }


}

|


end

p = [
  ["sqlite"  , "for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite>"],
  ["postgres", "for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>"],
  ["mysql"   , "for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>"],
  ["mssql"   , "TiberiusDecode"],
]

cc = p.combination(1) + p.combination(2) + p.combination(3) + p.combination(4)

all = ["sqlite", "postgres", "mysql", "mssql"]

full = head

cc.each do |c| 

  enabled_list = c.map{|a| a[0] } 
  disabled_list = all - enabled_list
  
  enabled = enabled_list.map{|f| "feature = \"#{f}\""}
  disabled = disabled_list.map{|f| "not(feature = \"#{f}\")"}
  rules = enabled + disabled
  cfgs = "#[cfg(all(#{rules.join(", ")}))]"

  wheres = c.map{|a| a[1]}
  wheres = wheres.join(" + ")

  full = full + "\n\n" + blocky(cfgs, wheres)
end


puts full

