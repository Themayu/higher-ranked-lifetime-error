use bevy_reflect::{Reflect, GetTypeRegistration};

#[derive(Reflect)]
pub struct ErrorStruct {
    #[reflect(ignore)]
    hkt: for<'a> fn(&'a str) -> &'a str,
}

impl Default for ErrorStruct {
    fn default() -> Self {
        ErrorStruct {
            hkt: |input| input
        }
    }
}

fn takes_hkt<T: GetTypeRegistration>() {}

fn lib() {
    takes_hkt::<ErrorStruct>();
}
