
// Caixa
#[allow(dead_code)]
pub struct Box {
  value:    f64,
  incomes:  Option<Vec<Income>>,
  expenses: Option<Vec<Expense>>,
}

#[allow(dead_code)]
pub struct DateValuePeriod {
  date:   String,
  value:  f64,
}

// Receitas
#[allow(dead_code)]
pub struct Income {
  name:             String,
  value:            f64,
  desc:             Option<String>,
  notes:            Option<Vec<String>>,
  period_dates:     Vec<DateValuePeriod>,
  date_created:     String,
  date_last_update: String,
  categories:       Option<Vec<String>>,
}

// Despesas
#[allow(dead_code)]
pub struct Expense {
  name:             String,
  value:            f64,
  desc:             Option<String>,
  notes:            Option<Vec<String>>,
  period_dates:     Vec<DateValuePeriod>,
  date_created:     String,
  date_last_update: String,
  categories:       Option<Vec<String>>,
  receipts:         Option<Vec<String>>,
}