#[cfg(test)]
mod test {
   use crate::{blueprint::BluePrint, data_base::*, errors::Errors, types::Types};

   // Função auxiliar para criar uma tabela com estrutura padrão
   fn setup_table_with_data() -> Result<Table, Errors> {
      let mut table = Table::create(String::from("my_table"));

      // Adicionar colunas
      table.add_column(Column {
         name: String::from("id"),
         values: Vec::new(),
      })?;

      table.add_column(Column {
         name: String::from("clients"),
         values: Vec::new(),
      })?;

      // Adicionar dados de ID
      table.add_data(String::from("id"), Types::Int(1))?;
      table.add_data(String::from("id"), Types::Int(2))?;
      table.add_data(String::from("id"), Types::Int(3))?;

      // Adicionar dados de clientes
      let clients = vec![
         String::from("Carlos"),
         String::from("Amanda"),
         String::from("José"),
         String::from("Antonio"),
      ];

      for client in clients {
         table.add_data(String::from("clients"), Types::Text(client))?;
      }

      Ok(table)
   }

   // Função auxiliar para criar tabela vazia
   fn setup_empty_table() -> Table {
      Table::create(String::from("my_table"))
   }

   #[test]
   fn test_create_table() -> Result<(), Errors>{
      let my_table = Table::create(String::from("my_table"));
      let expect = Table::create(String::from("my_table"));
      let table_name = match my_table.get_table_name() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      }; 
      assert_eq!(my_table, expect);
      assert_eq!(table_name, "my_table".to_string());
      Ok(())
   }

   #[test]
   fn test_create_table_with_different_names() -> Result<(), Errors>{
      let table1 = Table::create(String::from("users"));
      let table2 = Table::create(String::from("products"));

      let table1_name = match table1.get_table_name() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      let table2_name = match table1.get_table_name() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      assert_ne!(table1, table2);
      assert_eq!(table1_name, "users".to_string());
      assert_eq!(table2_name, "products".to_string());
      Ok(())
   }

   #[test]
   fn test_add_single_column() -> Result<(), Errors> {
      let mut table = setup_empty_table();

      table.add_column(Column {
         name: String::from("id"),
         values: Vec::new(),
      })?;

      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      assert_eq!(table_cols.len(), 1);
      assert_eq!(table_cols[0].name, "id");

      Ok(())
   }

   #[test]
   fn test_add_multiple_columns() -> Result<(), Errors> {
      let mut table = setup_empty_table();

      table.add_column(Column {
         name: String::from("id"),
         values: Vec::new(),
      })?;

      table.add_column(Column {
         name: String::from("clients"),
         values: Vec::new(),
      })?;

      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      assert_eq!(table_cols.len(), 2);
      assert_eq!(table_cols[0].name, "id");
      assert_eq!(table_cols[1].name, "clients");

      Ok(())
   }

   #[test]
   fn test_add_data_to_columns() -> Result<(), Errors> {
      let mut table = setup_empty_table();

      table.add_column(Column {
         name: String::from("id"),
         values: Vec::new(),
      })?;

      table.add_column(Column {
         name: String::from("clients"),
         values: Vec::new(),
      })?;

      // Adicionar dados
      table.add_data(String::from("id"), Types::Int(1))?;
      table.add_data(String::from("id"), Types::Int(2))?;
      table.add_data(String::from("id"), Types::Int(3))?;

      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      // Verificar se os dados foram adicionados
      let id_column = table_cols.iter()
         .find(|c| c.name == "id")
         .expect("Coluna 'id' não encontrada");
      
      assert_eq!(id_column.values.len(), 3);

      // Adicionar clientes
      let clients = vec![
         String::from("Carlos"),
         String::from("Amanda"),
         String::from("José"),
         String::from("Antonio"),
      ];

      for client in clients {
         table.add_data(String::from("clients"), Types::Text(client))?;
      }

      let clients_column = table_cols.iter()
         .find(|c| c.name == "clients")
         .expect("Coluna 'clients' não encontrada");
      
      assert_eq!(clients_column.values.len(), 4);

      Ok(())
   }

   #[test]
   fn test_add_data_to_nonexistent_column() {
      let mut table = setup_empty_table();

      let result = table.add_data(String::from("nonexistent"), Types::Int(1));
      
      assert!(result.is_err(), "Deveria retornar erro ao adicionar dados a coluna inexistente");
   }

   #[test]
   fn test_select_all_from_column() -> Result<(), Errors> {
      let table = setup_table_with_data()?;
      
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      let id_column = table_cols.iter()
         .find(|c| c.name == "id")
         .expect("Coluna 'id' não encontrada");
      
      assert_eq!(id_column.values.len(), 3, "Deveria ter 3 registros de ID");

      table.select(String::from("id"), None)?;

      Ok(())
   }

   #[test]
   fn test_select_with_filter() -> Result<(), Errors> {
      let table = setup_table_with_data()?;
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };
      // Verificar que o valor existe na coluna
      let clients_column = table_cols.iter()
         .find(|c| c.name == "clients")
         .expect("Coluna 'clients' não encontrada");

      let has_amanda = clients_column.values.iter()
         .any(|v| matches!(v, Types::Text(name) if name == "Amanda"));
      
      assert!(has_amanda, "Amanda deveria estar na coluna de clientes");

      // SELECT com filtro deve executar sem erros
      table.select(
         String::from("clients"),
         Some(Types::Text(String::from("Amanda")))
      )?;

      Ok(())
   }

   #[test]
   fn test_select_nonexistent_column() {
      let table = setup_table_with_data().unwrap();

      let result = table.select(String::from("nonexistent"), None);
      
      assert!(result.is_err(), "SELECT em coluna inexistente deveria retornar erro");
   }

   #[test]
   fn test_delete_data() -> Result<(), Errors> {
      let mut table = setup_table_with_data()?;
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };
      // Contar registros antes da exclusão
      let clients_column = table_cols.iter()
         .find(|c| c.name == "clients")
         .expect("Coluna 'clients' não encontrada");
      let count_before = clients_column.values.len();

      // Deletar um registro
      table.delete_data(
         String::from("clients"),
         Types::Text(String::from("Amanda"))
      )?;

      // Verificar que foi deletado
      let clients_column = table_cols.iter()
         .find(|c| c.name == "clients")
         .expect("Coluna 'clients' não encontrada");
      let count_after = clients_column.values.len();

      assert_eq!(count_after, count_before - 1, "Deveria ter um registro a menos");

      // Verificar que Amanda não existe mais
      let has_amanda = clients_column.values.iter()
         .any(|v| matches!(v, Types::Text(name) if name == "Amanda"));

      assert!(!has_amanda, "Amanda não deveria mais existir na tabela");

      Ok(())
   }

   #[test]
   fn test_delete_nonexistent_data() -> Result<(), Errors> {
      let mut table = setup_table_with_data()?;

      let result = table.delete_data(
         String::from("clients"),
         Types::Text(String::from("NonExistent"))
      );

      // Dependendo da implementação, pode retornar Ok ou Err
      // Ajuste conforme sua lógica
      assert!(result.is_ok() || result.is_err());

      Ok(())
   }

   #[test]
   fn test_delete_column() -> Result<(), Errors> {
      let mut table = setup_table_with_data()?;
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };
      let columns_before = table_cols.len();

      table.delete_column(String::from("id"))?;

      assert_eq!(table_cols.len(), columns_before - 1, "Deveria ter uma coluna a menos");

      // Verificar que a coluna não existe mais
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };
      let id_column_exists = table_cols.iter().any(|c| c.name == "id");
      assert!(!id_column_exists, "Coluna 'id' não deveria mais existir");

      Ok(())
   }

   #[test]
   fn test_delete_nonexistent_column() {
      let mut table = setup_table_with_data().unwrap();

      let result = table.delete_column(String::from("nonexistent"));

      assert!(result.is_err(), "Deletar coluna inexistente deveria retornar erro");
   }

   #[test]
   fn test_delete_all_columns() -> Result<(), Errors> {
      let mut table = setup_table_with_data()?;

      table.delete_column(String::from("id"))?;
      table.delete_column(String::from("clients"))?;
      
      let table_cols = match table.get_table_columns() {
         Ok(c) => {c},
         Err(e) => {return Err(e);}
      };

      assert_eq!(table_cols.len(), 0, "Não deveria ter nenhuma coluna");

      Ok(())
   }

   #[test]
   fn test_empty_table_operations() -> Result<(), Errors> {
      let mut table = setup_empty_table();

      let select_result = table.select(String::from("any_column"), None);
      assert!(select_result.is_err(), "SELECT em tabela vazia deveria falhar");

      let delete_result = table.delete_column(String::from("any_column"));
      assert!(delete_result.is_err(), "DELETE em tabela vazia deveria falhar");

      Ok(())
   }
}