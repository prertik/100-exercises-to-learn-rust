// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

fn product_name_validator(product_name: String) -> String {
    if product_name.is_empty() {
        panic!("Product name can't be empty.")
    } else if product_name.len() > 300 {
        panic!("Product name can't be longer than 300 bytes.")
    } else {
        product_name
    }
}

fn quantity_validator(quantity: u16) -> u16 {
    if quantity <= 0 {
        panic!("Quantity must be strictly greater than zero.")
    } else {
        quantity
    }
}

fn unit_price_validator(unit_price: u16) -> u16 {
    if unit_price <= 0 {
        panic!("Quantity must be strictly greater than zero.")
    } else {
        unit_price
    }
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u16) -> Order {
        let validated_product_name = product_name_validator(product_name);
        let validated_quantity = quantity_validator(quantity);
        let validated_unit_price = unit_price_validator(unit_price);

        Order {
            product_name: validated_product_name,
            quantity: validated_quantity,
            unit_price: validated_unit_price,
        }
    }

    pub fn total(&self) -> u16 {
        let total = &self.unit_price * &self.quantity;
        total
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        let validated_product_name = product_name_validator(new_product_name);
        self.product_name = validated_product_name
    }

    pub fn set_quantity(&mut self, new_quantity: u16) {
        let validated_quantity = quantity_validator(new_quantity);
        self.quantity = validated_quantity
    }

    pub fn set_unit_price(&mut self, new_unit_price: u16) {
        let validated_unit_price = unit_price_validator(new_unit_price);
        self.unit_price = validated_unit_price
    }
}