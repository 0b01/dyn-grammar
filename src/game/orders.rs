use crate::prelude::*;
use crate::burger::*;

pub struct Orders {
    pub orders: Option<[Burger; 10]>,
    pub order_result: [bool; 10],
    pub selected: usize,
}

impl Orders {
    pub fn new() -> Self {
        let orders = None;
        let order_result = [false; 10];
        Self {
            orders,
            selected: 0,
            order_result,
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        self.draw_orders(window, ing)?;
        self.draw_selected_burger(window, ing)?;
        Ok(())
    }

    pub fn draw_selected_burger(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        if self.orders.is_none() { return Ok(()) }
        let burger = &self.orders.as_ref().unwrap()[self.selected];
        burger.draw_order(window, ing)?;
        Ok(())
    }

    pub fn draw_orders(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        let success_img = ing.get_img("ordersuccess").unwrap();
        let fimg = ing.get_img("order").unwrap();
        let selected = ing.get_img("orderselect").unwrap();
        for i in 0..10 {

            let image = ing.get_img(&format!("{}", i+1)).unwrap();
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(250. + 50. * i as f32, 30.),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(0.5, 0.5)),
                100,
            );

            let image = if i == self.selected { selected }
                    else if self.order_result[i] { success_img }
                    else { fimg };

            window.draw_ex(&
                Rectangle::new(
                    Vector::new(250. + 50. * i as f32, 30.),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(1.5, 1.5)),
                100,
            );
        }
        Ok(())
    }

    pub fn mouse_move(&mut self, v: &Vector) -> Option<usize> {
        for i in 0..10 {
            if (v.y > 28.453894 && v.y < 66.80449)
            && v.x > (250.59584 + i as f32 * 50.)
            && v.x < (281.54004 + i as f32 * 50.) {
                self.selected = i;
                return Some(i);
            }
        }
        None
    }

    pub fn set_success(&mut self, i: usize) {
        self.order_result[i] = true;
    }

    pub fn clear_result(&mut self) {
        self.order_result = [false; 10];
    }

}