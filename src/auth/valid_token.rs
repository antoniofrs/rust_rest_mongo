#[derive(Debug, Clone)]
pub struct ValidToken {
    pub permissions: Vec<String>,
}

impl ValidToken {

    pub fn has_permission(&self, permission: &str) -> Result<(), ()> {
        self.has_permissions(&[permission])
    }
    pub fn has_permissions(&self, required_permissions: &[&str]) -> Result<(), ()> {
        for &perm in required_permissions {
            if !self.permissions.contains(&perm.to_string()) {
                return Err(());
            }
        }
        Ok(())
    }
}