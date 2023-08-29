use diesel::table;
table!{
    employees(id){
        id->Integer,
        first_name->Varchar,
        last_name->Varchar,
        department->Varchar,
        salary->Integer,
        age->Integer,
    }
}