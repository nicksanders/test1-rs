#[allow(dead_code, unused_variables)]

use std::collections::BTreeMap;

fn main() {

    #[allow(dead_code)]
    enum ColType {
        Boolean,
        Integer,
        BigInteger,
        Float,
        Numeric,
        Enum,
        Date,
        Datetime,
        String,
        Text,
    }

    #[allow(dead_code)]
    enum Op {
        Equal,
        NotEqual,
        GreatThan,
        LessThan,
        GreatThanOrEqual,
        LessThanOrEqual,
    }

    struct Col<'a> {
        name: &'a str,
        col_type: ColType,
        length: Option<usize>,
        null: bool,
    }

    struct Tbl<'a> {
        name: &'a str,
        cols: BTreeMap<&'a str, Col<'a>>,
    }

    struct Query<'a> {
        table: &'a Tbl<'a>,
        where_sql: Vec<String>,
        args: Vec<String>,
    }

    impl <'a>Tbl<'a> {
        fn new(name: &'a str, cols: Vec<Col<'a>>) -> Self {
            let mut table = Tbl{name: name, cols: BTreeMap::new()};
            for col in cols {
                table.cols.insert(col.name, col);
            }
            table
        }

        fn query(&'a self) -> Query {
            Query{table: self, where_sql: vec![], args: vec![]}
        }

        fn to_sql(&self) -> String {
            format!("CREATE TABLE {0} {{ {1} }};", self.name, self.cols_to_sql())
        }

        fn cols_to_sql(&self) -> String {
            let mut v = vec![];
            for (_, col) in &self.cols {
                v.push(col.to_sql());
            }
            v.connect(", ")
        }
    }

    impl <'a>Col<'a> {
        fn new(name: &'a str, col_type: ColType) -> Self {
            Col{name: name, col_type: col_type, null: false, length: None}
        }

        fn length(mut self, length: usize) -> Self {
            self.length = Some(length);
            self
        }

        fn allow_null(mut self) -> Self {
            self.null = true;
            self
        }

        fn to_sql(&self) -> String {
            let data_type = match self.col_type {
                ColType::Boolean => "BOOLEAN",
                ColType::Integer => "INT",
                ColType::BigInteger => "BIGINT",
                ColType::Float => "FLOAT",
                ColType::Numeric => "NUMERIC",
                ColType::Enum => "ENUM",
                ColType::Date => "DATE",
                ColType::Datetime => "DATETIME",
                ColType::String => "VARCHAR",
                ColType::Text => "TEXT",
            };
            let length_str = match self.length {
                Some(v) => format!("({})", v),
                None => "".to_string(),
            };
            let null_str = match self.null {
                true => "NULL",
                false => "NOT NULL",
            };
            format!("{} {}{} {}", self.name, data_type, length_str, null_str)
        }
    }

    impl <'a>Query<'a> {
        fn filter(mut self, col_name: &str, op: Op, value: &str) -> Self {
            let op_str = match op {
                Op::Equal => "=",
                Op::NotEqual => "!=",
                Op::GreatThan => ">",
                Op::LessThan => "<",
                Op::GreatThanOrEqual => ">=",
                Op::LessThanOrEqual => "<=",
            };
            self.where_sql.push(format!("{} {} ?", col_name, op_str));
            self.args.push(value.to_string());
            self
        }
        fn to_sql(&self) -> String {
            let mut s = format!("SELECT * FROM {}", self.table.name);
            if self.where_sql.len() > 0 {
                s = format!("{} WHERE {}", s, self.where_sql.connect(" AND "));
            }
            format!("{};", s)
        }
    }

    let t1 = Tbl::new("test1", vec![
        Col::new("col1", ColType::Integer),
        Col::new("col2", ColType::String).allow_null(),
        Col::new("col3", ColType::String).length(255).allow_null(),
    ]);

    println!("{}", t1.to_sql());
    println!("{}", t1.query().to_sql());
    println!("{}", t1.query().filter("col1", Op::GreatThan, "14").to_sql());
}
