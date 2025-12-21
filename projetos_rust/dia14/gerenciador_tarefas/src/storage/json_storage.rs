use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

/// Gerenciador de armazenamento em JSON
pub struct Storage {
    file_path: String,
}

impl Storage {
    /// Cria uma nova instância de Storage
    pub fn new(file_path: String) -> Self {
        Storage { file_path }
    }

    /// Salva dados em arquivo JSON
    pub fn save<T>(&self, data: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        // Serializa para JSON com formatação bonita
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Cria o diretório se não existir
        if let Some(parent) = Path::new(&self.file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Escreve no arquivo
        fs::write(&self.file_path, json)?;

        Ok(())
    }

    /// Carrega dados do arquivo JSON
    pub fn load<T>(&self) -> io::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Verifica se o arquivo existe
        if !Path::new(&self.file_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Arquivo não encontrado",
            ));
        }

        // Lê o arquivo
        let json = fs::read_to_string(&self.file_path)?;

        // Deserializa o JSON
        let data = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(data)
    }

    /// Verifica se o arquivo existe
    pub fn exists(&self) -> bool {
        Path::new(&self.file_path).exists()
    }

    /// Deleta o arquivo de armazenamento
    pub fn delete(&self) -> io::Result<()> {
        if self.exists() {
            fs::remove_file(&self.file_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_save_and_load() {
        let storage = Storage::new("data/test_storage.json".to_string());

        let data = TestData {
            id: 1,
            name: "Test".to_string(),
        };

        // Salva
        storage.save(&data).unwrap();

        // Carrega
        let loaded: TestData = storage.load().unwrap();

        assert_eq!(data, loaded);

        // Limpa
        storage.delete().unwrap();
    }

    #[test]
    fn test_load_nonexistent_file() {
        let storage = Storage::new("data/nonexistent.json".to_string());

        let result: io::Result<TestData> = storage.load();

        assert!(result.is_err());
    }

    #[test]
    fn test_exists() {
        let storage = Storage::new("data/test_exists.json".to_string());

        assert!(!storage.exists());

        let data = TestData {
            id: 1,
            name: "Test".to_string(),
        };
        storage.save(&data).unwrap();

        assert!(storage.exists());

        storage.delete().unwrap();
        assert!(!storage.exists());
    }
}