fn main() {
    print!("=== CSV ===\n\n");
    csv::csvparse();
    print!("\n\n=== numbers ===\n\n");
    numbers::numbersparse();
    print!("\n\n=== INI ===\n\n");
    ini::iniparse();
}

mod csv {
    use pest::Parser;
    use pest_derive::Parser;
    use std::fs;

    #[derive(Parser)]
    #[grammar = "csv.pest"]
    pub struct CSVParser;

    pub fn csvparse() {
        // let successful_parse = CSVParser::parse(Rule::field, "-150.0");
        // println!("{:?}", successful_parse);

        // let failed_parse = CSVParser::parse(Rule::field, "Hello there");
        // println!("{:?}", failed_parse);

        let unparsed_file = fs::read_to_string("numbers.csv").expect("cannot read file");

        let file = CSVParser::parse(Rule::file, &unparsed_file)
            .expect("Unsuccessful parse")
            .next()
            .unwrap();

        let mut field_sum: f64 = 0.0;
        let mut record_count: u64 = 0;

        for record in file.into_inner() {
            match record.as_rule() {
                Rule::record => {
                    record_count += 1;

                    for field in record.into_inner() {
                        field_sum += field.as_str().parse::<f64>().unwrap();
                    }
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        println!("Sum of all fields: {}", field_sum);
        println!("Number of records: {}", record_count);
    }
}

mod numbers {
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "numbers.pest"]
    pub struct NumbersParser;

    pub fn numbersparse() {
        let res = NumbersParser::parse(Rule::sum, "123 + 456").unwrap();
        let tokens = res.tokens();

        for token in tokens {
            println!("{:?}", token)
        }

        let pair = NumbersParser::parse(Rule::enclosed, "(..123..)")
            .unwrap()
            .next()
            .unwrap();

        println!("Pair: {:?}", pair);
        println!("Pair: {:?}", pair.as_rule());
        println!("Pair: {:?}", pair.as_str());
        println!("Pair: {:?}", pair.as_node_tag());
        println!("Pair: {:?}", pair.as_span());

        let inner = pair.into_inner().next().unwrap();

        println!("Inner: {:?}", inner);
        println!("Inner: {:?}", inner.as_rule());
        println!("Inner: {:?}", inner.as_str());
        println!("Inner: {:?}", inner.as_node_tag());
        println!("Inner: {:?}", inner.as_span());

        let sum = NumbersParser::parse(Rule::sum, "567 + 8910");

        let pairs = sum.clone().unwrap().next().unwrap().into_inner();

        let numbers = pairs
            .clone()
            .map(|p| str::parse(p.as_str()).unwrap())
            .collect::<Vec<i32>>();

        println!("{:?}", numbers);

        let mut inner_rules = sum.clone().unwrap().next().unwrap().into_inner();

        let lhs = inner_rules.next().unwrap().as_str();
        let rhs = inner_rules.next().unwrap().as_str();

        println!("lhs: {}", lhs);
        println!("rhs: {}", rhs);
    }
}

mod ini {
    use std::{collections::HashMap, fs, hash::Hash};

    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "ini.pest"]
    pub struct INIParser;

    pub fn iniparse() {
        let unparsed_file = fs::read_to_string("example.ini").expect("Unable to load example.ini");

        let file = INIParser::parse(Rule::file, &unparsed_file)
            .expect("Error parsing the file")
            .next()
            .unwrap();

        println!("\n\n=== Parsed INI File ===");
        println!("{:#?}", file);

        let mut configuration: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

        let mut current_section = "";

        for line in file.into_inner() {
            match line.as_rule() {
                Rule::section => {
                    current_section = line.into_inner().next().unwrap().as_str();
                }
                Rule::rule => {
                    let mut i = line.into_inner();
                    let key = i.next().unwrap().as_str();
                    let value = i.next().unwrap().as_str();
                    configuration
                        .entry(current_section)
                        .or_default()
                        .insert(key, value);
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        println!("\n\n=== INI File Configuration ===");
        println!("{:#?}", configuration);
    }
}
