// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

struct DropBomb {
    armed: bool
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.armed {
            panic!("You've set us up the bomb")
        }
    }
}

impl DropBomb {
    pub fn new() -> Self {
        Self{armed: true}
    }
    pub fn disarm(&mut self) {
        self.armed = false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.disarm();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
