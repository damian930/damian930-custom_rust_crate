use my_project_parser_rogue_druid_on_hearthstone_matt_piper_formal_bound_duo_2025_enjoyer::list_parser;
  
pub fn main() {
      let parsed_data = list_parser::list("[1,2,3,4,5]");

      println!("{:?}", parsed_data);
      assert_eq!(parsed_data, Ok(vec![1,2,3,4,5]));
}