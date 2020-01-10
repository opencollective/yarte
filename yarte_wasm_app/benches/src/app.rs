use js_sys::Date;
use rand::{rngs::SmallRng, SeedableRng};
use serde_json::from_str;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{Element, Event};

use yarte_wasm_app::*;

use crate::{
    handler::*,
    row::{row_element, Row, RowDOM},
};

#[wasm_bindgen]
extern "C" {
    fn get_state() -> String;
}

pub struct NonKeyed {
    pub id: u32,
    pub data: Vec<Row>,
    pub selected: Option<u32>,
    pub rng: SmallRng,
    // Black box
    pub t_root: u8,
    old_selected: Option<usize>,
    tr: Element,
    tbody: Element,
    //
    pub tbody_children: Vec<RowDOM>,
    closure_create: Option<Closure<dyn Fn(Event)>>,
    closure_create_10: Option<Closure<dyn Fn(Event)>>,
    closure_append: Option<Closure<dyn Fn(Event)>>,
    closure_update: Option<Closure<dyn Fn(Event)>>,
    closure_clear: Option<Closure<dyn Fn(Event)>>,
    closure_swap: Option<Closure<dyn Fn(Event)>>,
}

impl App for NonKeyed {
    type BlackBox = ();

    fn __render(&mut self, mb: &Mailbox<Self>) {
        if self.t_root & 0b0000_0001 != 0 {
            let dom_len = self.tbody_children.len();
            let row_len = self.data.len();
            if dom_len < row_len {
                for (dom, row) in self
                    .tbody_children
                    .iter_mut()
                    .zip(self.data[..dom_len].iter())
                    .filter(|(dom, _)| dom.t_root != 0)
                {
                    dom.update(row, mb);
                }
                for row in self.data[dom_len..].iter() {
                    let v = RowDOM::new(row.id, &row.label, &self.tr, mb);
                    self.tbody.append_child(&v.root).unwrap_throw();
                    self.tbody_children.push(v);
                }
            } else if row_len < dom_len {
                for (dom, row) in self.tbody_children[..row_len]
                    .iter_mut()
                    .zip(self.data.iter())
                    .filter(|(dom, _)| dom.t_root != 0)
                {
                    dom.update(row, mb);
                }
                for _ in 0..dom_len - row_len {
                    self.tbody
                        .remove_child(&self.tbody.last_child().unwrap_throw())
                        .unwrap_throw();
                }
                self.tbody_children.drain(row_len..);
            } else {
                for (dom, row) in self
                    .tbody_children
                    .iter_mut()
                    .zip(self.data.iter())
                    .filter(|(dom, _)| dom.t_root != 0)
                {
                    dom.update(row, mb)
                }
            }
        }

        // TODO:  detect `selected`
        if self.t_root & 0b0000_0010 != 0 {
            let children = self.tbody.children();
            if let Some(old) = self.old_selected.take() {
                children.item(old as u32).unwrap_throw().set_class_name("")
            }
            if let Some(new) = self.selected {
                if let Some(new) = self.data.iter().position(|x| x.id == new) {
                    children
                        .item(new as u32)
                        .unwrap_throw()
                        .set_class_name("danger");
                    self.old_selected = Some(new);
                } else {
                    self.selected = None;
                }
            }
        }

        self.t_root = 0;
    }

    fn __hydrate(&mut self, mb: &Mailbox<Self>) {
        let document = web_sys::window().unwrap_throw().document().unwrap_throw();
        let f = document
            .body()
            .unwrap_throw()
            .first_child()
            .unwrap_throw()
            .first_child()
            .unwrap_throw(); // div.jumbotron

        let f_0 = f.first_child().unwrap_throw(); // div.row
        let f_0_0 = f_0.first_child().unwrap_throw(); // div.col-md-6
        let f_0_1 = f_0_0.next_sibling().unwrap_throw(); // div.col-md-6
        let f_0_1_0 = f_0_1.first_child().unwrap_throw(); // div.row

        let f_0_1_0_0 = f_0_1_0.first_child().unwrap_throw(); // div.col-sm-6 smallpad
        let button_create = f_0_1_0_0.first_child().unwrap_throw(); // button CREATE 1_000
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Create);
        }) as Box<dyn Fn(Event)>);
        button_create
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_create = Some(cl);

        let f_0_1_0_1 = f_0_1_0_0.next_sibling().unwrap_throw(); // div.col-sm-6 smallpad
        let button_create_10 = f_0_1_0_1.first_child().unwrap_throw(); // button CREATE 10_000
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Create10);
        }) as Box<dyn Fn(Event)>);
        button_create_10
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_create_10 = Some(cl);

        let f_0_1_0_2 = f_0_1_0_1.next_sibling().unwrap_throw(); // div.col-sm-6 smallpad
        let button_append = f_0_1_0_2.first_child().unwrap_throw(); // button  APPEND 1_000
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Append)
        }) as Box<dyn Fn(Event)>);
        button_append
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_append = Some(cl);

        let f_0_1_0_3 = f_0_1_0_2.next_sibling().unwrap_throw(); // div.col-sm-6 smallpad
        let button_update = f_0_1_0_3.first_child().unwrap_throw(); // button  UPDATE
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Update);
        }) as Box<dyn Fn(Event)>);
        button_update
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_update = Some(cl);

        let f_0_1_0_4 = f_0_1_0_3.next_sibling().unwrap_throw(); // div.col-sm-6 smallpad
        let button_clear = f_0_1_0_4.first_child().unwrap_throw(); // button CLEAR
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Clear);
        }) as Box<dyn Fn(Event)>);
        button_clear
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_clear = Some(cl);

        let f_0_1_0_5 = f_0_1_0_4.next_sibling().unwrap_throw(); // div.col-sm-6 smallpad
        let button_swap = f_0_1_0_5.first_child().unwrap_throw(); // button SWAP
        let cloned = mb.clone();
        let cl = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            cloned.send(Swap)
        }) as Box<dyn Fn(Event)>);
        button_swap
            .add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure_swap = Some(cl);

        assert_eq!(self.tbody_children.len(), self.data.len());

        // hydrate Each
        for (dom, row) in self.tbody_children.iter_mut().zip(self.data.iter()) {
            dom.hydrate(row, mb);
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct InitialState {
    #[serde(default)]
    data: Vec<Row>,
    #[serde(default)]
    id: u32,
    #[serde(default)]
    selected: Option<u32>,
}

// Construct pre hydrate App
impl Default for NonKeyed {
    fn default() -> Self {
        let state = get_state();
        let InitialState { data, id, selected }: InitialState =
            from_str(&state).unwrap_or_default();
        let doc = web_sys::window().unwrap_throw().document().unwrap_throw();
        let body = doc.body().unwrap_throw();
        let node = body.first_element_child().unwrap_throw();
        let f = node.first_element_child().unwrap_throw(); // div.jumbotron
        let n1 = f.next_element_sibling().unwrap_throw(); // table.table table-hover table-striped test-data
        let tbody = n1.first_element_child().unwrap_throw(); // tbody

        let mut tbody_children = vec![];
        if let Some(mut curr) = tbody.first_child() {
            loop {
                let id_node = curr.first_child().unwrap_throw();
                let label_parent = id_node.next_sibling().unwrap_throw();
                let label_node = label_parent.first_child().unwrap_throw();
                let delete_parent = label_parent.next_sibling().unwrap_throw();
                let delete_node = delete_parent.first_child().unwrap_throw();

                curr = if let Some(new) = curr.next_sibling() {
                    tbody_children.push(RowDOM {
                        t_root: 0,
                        root: curr,
                        id_node,
                        label_node,
                        delete_node,
                        closure_select: None,
                        closure_delete: None,
                    });

                    new
                } else {
                    tbody_children.push(RowDOM {
                        t_root: 0,
                        root: curr,
                        id_node,
                        label_node,
                        delete_node,
                        closure_select: None,
                        closure_delete: None,
                    });

                    break;
                }
            }
        }

        Self {
            // state template variables
            id,
            data,
            selected,
            // state variable
            rng: SmallRng::seed_from_u64(Date::now() as u64),
            // Black box
            t_root: 0,
            old_selected: None,
            tbody,
            tbody_children,
            tr: row_element(),
            closure_create: None,
            closure_create_10: None,
            closure_append: None,
            closure_update: None,
            closure_clear: None,
            closure_swap: None,
        }
    }
}
