use crate::vending_machine::ProductMap;

mod vending_machine;

mod money;

fn main() {
    let product = ProductMap::new("Pepsi", 25);
    println!("{product}")
}

#[cfg(test)]
mod tests {
    use crate::{
        money::{Money, DIME, NICKEL, TWO_DIMES},
        vending_machine::{Product, ProductMap, VendingError, VendingMachine},
    };

    #[test]
    fn test_purchase_success() {
        let mut machine = VendingMachine::new();

        machine.fill(
            &[
                ProductMap::new("Skittles", 35),
                ProductMap::new("Monster", 45),
                ProductMap::new("Mentos", 24),
            ],
            85,
        );

        let mut expected_money = Money::new();
        expected_money.increase(NICKEL, 1).increase(DIME, 1);
        assert_eq!(
            Ok((ProductMap::new("Skittles", 35), expected_money)),
            machine.purchase(&Product::new("Skittles"), 50)
        );
    }

    #[test]
    fn test_purchase_not_found() {
        let mut machine = VendingMachine::new();

        machine.fill(
            &[
                ProductMap::new("Skittles", 35),
                ProductMap::new("Monster", 45),
                ProductMap::new("Mentos", 24),
            ],
            85,
        );

        assert_eq!(
            Err(VendingError::NoProduct(Product::new("Pepsi"))),
            machine.purchase(&Product::new("Pepsi"), 50)
        );
    }

    #[test]
    fn test_purchase_not_enough_money() {
        let mut machine = VendingMachine::new();

        let mut money = Money::new();
        money.increase(TWO_DIMES, 2).increase(DIME, 1);
        machine.fill(&[ProductMap::new("Monster", 45)], money.clone());

        assert_eq!(
            Err(VendingError::NotEnoughMoney {
                money: 30.into(),
                product: ProductMap::new("Monster", 45)
            }),
            machine.purchase(&Product::new("Monster"), 30)
        );
    }

    #[test]
    fn test_purchase_no_change() {
        let mut machine = VendingMachine::new();

        let mut money = Money::new();
        money.increase(TWO_DIMES, 2).increase(DIME, 1);
        machine.fill(&[ProductMap::new("Monster", 45)], money.clone());

        let mut expected_change = Money::new();
        expected_change.increase(NICKEL, 1);

        assert_eq!(
            Err(VendingError::NoChange {
                change: expected_change,
                money: money.clone()
            }),
            machine.purchase(&Product::new("Monster"), 50)
        );
    }
}
