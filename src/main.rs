#[derive(PartialEq, Debug, Clone)]
enum PaymentType {
    DigitalToken,
    Cash
}

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32
}

struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32
}

struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    fn add_member(&mut self, member: Buyer) {
        self.members.push(member);
    }

    fn find_buyer(&self, payment: &PaymentType) -> isize {
        for (index, member) in self.members.iter().enumerate() {
            if member.payment_type == *payment {
                return index as isize;
            }
        }
        -1
    }

    fn buy(&mut self, buyer_index: usize, seller: &mut Seller) {
        while self.members[buyer_index].balance >= seller.price {
            self.members[buyer_index].balance -= seller.price;
            seller.balance += seller.price;
        }
    }
}

fn main() {
    let john = Buyer {
        name: String::from("John"),
        payment_type: PaymentType::DigitalToken,
        balance: 100.0
    };
    let jane = Buyer {
        name: String::from("Jane"),
        payment_type: PaymentType::Cash,
        balance: 100.0
    };

    let mut group = BuyerGroup {
        members: Vec::new()
    };

    group.add_member(john);
    group.add_member(jane);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0
    };
    let sally_index = group.find_buyer(&PaymentType::Cash);
    if sally_index != -1 {
        group.buy(sally_index as usize, &mut seller);
    }
}