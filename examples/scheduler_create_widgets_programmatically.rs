use ez_term::*;
use std::collections::HashMap;

fn main() {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    // Let's create some mock SQL records to spawn widgets from
    let mut sql_records = Vec::new();
    for x in 1..=100 {
        let mut sql_record = HashMap::new();
        sql_record.insert("id", format!("{}", x));
        sql_record.insert("name", format!("Record {}", x));
        sql_record.insert("date", format!("{}-{}-2022", (x / 31) + 1, x % 31 + 1));
        sql_records.push(sql_record);
    }

    // Now we'll iterate over the records and spawn a template for each one
    let template_name = "SqlRecord";
    let parent_id = "sql_records_layout";
    for (i, sql_record) in sql_records.iter().enumerate() {
        let new_id = format!("record_{}", i);

        let (new_widget, mut new_states) =
            scheduler.prepare_create_widget(template_name, &new_id, parent_id, &mut state_tree);

        // We will modify the widget state of each label to reflect the SQL record
        new_states
            .get_mut("record_id")
            .as_label_mut()
            .set_text(sql_record.get("id").unwrap().to_string());
        new_states
            .get_mut("record_name")
            .as_label_mut()
            .set_text(sql_record.get("name").unwrap().to_string());
        new_states
            .get_mut("record_date")
            .as_label_mut()
            .set_text(sql_record.get("date").unwrap().to_string());

        scheduler.create_widget(new_widget, new_states, &mut state_tree);
    }
    run(root_widget, state_tree, scheduler);
}
