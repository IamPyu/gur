pub struct Package {
    pub giturl: String,
    pub dependencies: Vec<Package>,
    pub buildtask: String
}
