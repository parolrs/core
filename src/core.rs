//! Core of parol.rs.

extern crate serde_json;

/// Principal component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parol {
    application: String,
    username: String,
    password: String,
    notes: String,
}

impl Parol {
    /// Constructs a new, empty ```Parol```.
    ///
    /// # Examples
    /// ```ignore
    /// let parol = Parol::new();
    /// ```
    pub fn new() -> Parol {
        Parol {
            application: String::new(),
            username: String::new(),
            password: String::new(),
            notes: String::new(),
        }
    }

    /// Constructs a new, ```Parol``` with application, username, password and notes.
    ///
    /// # Arguments
    /// * `application` - Name of the application/url/etc.
    /// * `username` - Username/ID/Email/etc.
    /// * `password` - Password/Code/etc.
    /// * `notes` - A notes, maybe a BIN or anything else ? No ? pass a empty &str ;).
    ///
    /// # Examples
    /// ```ignore
    /// let twitter = Parol::new_with_arguments("twitter", "Ogromny", "super_strong_password", "rescue email: blabla@topmail.ru");
    /// ```
    pub fn new_with_arguments(application: &str, username: &str, password: &str, notes: &str) -> Parol {
        Parol {
            application: String::from(application),
            username: String::from(username),
            password: String::from(password),
            notes: String::from(notes),
        }
    }

    /// Returns application name.
    pub fn get_application(&self) -> String {
        self.application.clone()
    }

    /// Modifies the application name.
    ///
    /// # Arguments
    /// * `application` - New application name.
    ///
    /// # Examples
    /// ```ignore
    /// let twitter = Parol::new();
    /// twitter.set_application("twitter");
    /// assert_eq!(twitter.get_application(), "twitter");
    /// ```
    pub fn set_application(&mut self, application: &str) {
        self.application = String::from(application);
    }

    /// Returns username.
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    /// Modifies the username.
    ///
    /// # Arguments
    /// * `username` - New username.
    ///
    /// # Examples
    /// ```ignore
    /// let twitter = Parol::new();
    /// twitter.set_username("Ogromny");
    /// assert_eq!(twitter.get_username(), "Ogromny");
    /// ```
    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }

    /// Returns password.
    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    /// Modifies the password.
    ///
    /// # Arguments
    /// * `password` - New password.
    ///
    /// # Examples
    /// ```ignore
    /// let twitter = Parol::new();
    /// twitter.set_password("super_strong_password");
    /// assert_eq!(twitter.get_password(), "super_strong_password");
    /// ```
    pub fn set_password(&mut self, password: &str) {
        self.password = String::from(password);
    }

    /// Returns notes.
    pub fn get_notes(&self) -> String {
        self.notes.clone()
    }

    /// Modifies the notes.
    ///
    /// # Arguments
    /// * `notes` - New notes.
    ///
    /// # Examples
    /// ```ignore
    /// let twitter = Parol::new();
    /// twitter.set_notes("Rescue email: blabla@topmail.ru");
    /// assert_eq!(twitter.get_notes(), "Rescue email: blabla@topmail.ru");
    /// ```
    pub fn set_notes(&mut self, notes: &str) {
        self.notes = String::from(notes);
    }
}

/// ```Parol```s containers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Parols {
    parols: Vec<Parol>,
    len: usize,
}

impl Parols {
    /// Constructs a new, empty ```Parols```.
    ///
    /// # Examples
    /// ```ignore
    /// let parols = Parols::new();
    /// ```
    pub fn new() -> Parols {
        Parols {
            parols: Vec::new(),
            len: 0,
        }
    }

    /// Constructs a new, ```Parols``` from a database.
    ///
    /// # Arguments
    /// * `database` - A Vector of Parol.
    ///
    /// # Examples
    /// ```ignore
    /// let mut vector_of_parol = Vec::new();
    /// for i in 0 .. 10 {
    /// vector_of_parol.push(Parol::new());
    /// }
    /// let parols = Parols::new_from_database(vector_of_parol);
    /// ```
    pub fn new_from_database(database: Vec<Parol>) -> Parols {
        let len = database.len();
        Parols {
            parols: database,
            len: len,
        }
    }

    /// Constructs a new, ```Parols``` from a json.
    ///
    /// # Arguments
    /// * `json` - A database in json format.
    pub fn new_from_json(json: &str) -> Parols {
        match serde_json::from_str(json) {
            Ok(parols) => parols,
            Err(err) => panic!("{}", err),
        }
    }

    /// Returns the parol at index ```id``` or None if ```id``` does not exist.
    ///
    /// # Arguments
    /// * `id` - ID of the desired Parol.
    ///
    /// # Examples
    /// ```ignore
    /// match parols.get(5) {
    ///     Some(p) => println!("{}", p),
    ///     None => {},
    /// }
    /// ```
    pub fn get(&self, id: usize) -> Option<Parol> {
        if self.len <= id {
            return None
        } else {
            return Some(self.parols[id].clone());
        }
    }

    /// Change the parol at id ```id``` by ```parol```, Return Ok() if success, else return Err(String).
    ///
    /// # Arguments
    /// * `id` - if of the parol
    /// * `parol` - new Parol
    ///
    /// # Examples
    /// ```ignore
    /// match parols.set(5, parol) {
    ///     Ok(()) => {},
    ///     Err(err) => println!("err = {:?}", err),
    /// }
    /// ```
    pub fn set(&mut self, id: usize, parol: Parol) -> Result<(), String> {
        if self.len <= id {
            return Err(format!("index out of bounds: the len is {} but the index is {}", self.len, id));
        } else {
            self.parols[id] = parol;
            self.len = self.parols.len();
        }
        Ok(())
    }

    /// Append a new Parol and return his id
    ///
    /// # Arguments
    /// * `parol` - the Parol to append
    ///
    /// # Exemples
    /// ```ignore
    /// parols.push(Parol::new());
    /// ```
    pub fn push(&mut self, parol: Parol) -> usize {
        self.parols.push(parol);
        self.len = self.parols.len();
        self.len
    }

    /// Remove the parol at id ```id``` and return the Parol or an error
    ///
    /// # Arguments
    /// * `id` - id of the Parol
    ///
    /// # Examples
    /// ```ignore
    /// match ps.remove(7) {
    ///     Ok(parol) => println!("parol = {:?}", parol),
    ///     Err(err) => println!("err = {:?}", err),
    /// }
    /// ```
    pub fn remove(&mut self, id: usize) -> Result<Parol, String> {
        if self.len <= id {
            return Err(format!("index out of bounds: the len is {} but the index is {}", self.len, id));
        } else {
            let n = self.parols.remove(id);
            self.len = self.parols.len();
            return Ok(n)
        }
    }

    /// Return the ```Parols``` in json format.
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(err) => panic!("{}", err),
        }
    }

    /// Return the number of ```Parol``` contained in the ```Parols```.
    pub fn len(&self) -> usize {
        self.len
    }
}
