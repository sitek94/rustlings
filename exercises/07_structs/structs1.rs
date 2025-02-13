struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
}

// TODO: Add the fields that the test `tuple_structs` expects
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.

    struct User {
        active: bool,
        first_name: String,
        city: String,
    }

    let user1 = User {
        active: true,
        first_name: String::from("Maciek"),
        city: String::from("Wroclaw"),
    };

    let user2 = User {
        first_name: String::from("Jan"),
        ..user1
    };

    println!(
        "{} is {} and lives in ...",
        user1.first_name,
        if user1.active { "active" } else { "not-active" },
        // user1.city // Error cause `city` was moved!
    );

    println!(
        "{} is {} and lives in {}",
        user2.first_name,
        if user2.active { "active" } else { "not-active" },
        user2.city // `city` was moved here!
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
