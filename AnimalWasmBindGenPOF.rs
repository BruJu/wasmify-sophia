/* ============================================================================
 * Animal general implementation
 */

pub trait Animal {
    fn cri(&self);
}

pub struct Chat { }

pub struct Chien {}

impl Animal for Chat {
    fn cri(&self) {
        log("Maou");
    }
}

impl Animal for Chien {
    fn cri(&self) {
        log("Ouaffe");
    }
}

// We want to propose to the users a class Animal that can provide the services
// of Chien and Chat;


/* ============================================================================
 * Proposed solution
 */

// Enum to contain every posible members
pub enum AnimalEnum {
    Chat(Chat),
    Chien(Chien)
}

// Wrapper of the enum because we can't bind functions to enum
#[wasm_bindgen]
pub struct AnimalJS {
    animal_enum: AnimalEnum
}

// Dispatcher of the functions calls
#[wasm_bindgen]
impl AnimalJS {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> AnimalJS {
        // new can creates either cats or dogs (decided at execution time)
        AnimalJS {
            animal_enum: if val % 2 == 0 {
                AnimalEnum::Chat(Chat { })
            } else {
                AnimalEnum::Chien(Chien { })
            }
        }
    }

    #[wasm_bindgen]
    pub fn cri(&self) {
        // Dispatcher of the cri function
        match &self.animal_enum {
            AnimalEnum::Chat(chat) => chat.cri(),
            AnimalEnum::Chien(chien) => chien.cri()
        }
    }
}
