/// KeyEvent stores which key is down and which key is up.
pub struct KeyEvent {
    enter: bool,
    arrow_left: bool,
    arrow_up: bool,
    arrow_right: bool,
    arrow_down: bool,
    digit_0: bool,
    digit_1: bool,
    digit_2: bool,
    digit_3: bool,
    digit_4: bool,
    digit_5: bool,
    digit_6: bool,
    digit_7: bool,
    digit_8: bool,
    digit_9: bool,
    key_a: bool,
    key_b: bool,
    key_c: bool,
    key_d: bool,
    key_e: bool,
    key_f: bool,
    key_g: bool,
    key_h: bool,
    key_i: bool,
    key_j: bool,
    key_k: bool,
    key_l: bool,
    key_m: bool,
    key_n: bool,
    key_o: bool,
    key_p: bool,
    key_q: bool,
    key_r: bool,
    key_s: bool,
    key_t: bool,
    key_u: bool,
    key_v: bool,
    key_w: bool,
    key_x: bool,
    key_y: bool,
    key_z: bool,
}

impl KeyEvent {
    pub(crate) fn new() -> Self {
        Self {
            enter: false,
            arrow_left: false,
            arrow_up: false,
            arrow_right: false,
            arrow_down: false,
            digit_0: false,
            digit_1: false,
            digit_2: false,
            digit_3: false,
            digit_4: false,
            digit_5: false,
            digit_6: false,
            digit_7: false,
            digit_8: false,
            digit_9: false,
            key_a: false,
            key_b: false,
            key_c: false,
            key_d: false,
            key_e: false,
            key_f: false,
            key_g: false,
            key_h: false,
            key_i: false,
            key_j: false,
            key_k: false,
            key_l: false,
            key_m: false,
            key_n: false,
            key_o: false,
            key_p: false,
            key_q: false,
            key_r: false,
            key_s: false,
            key_t: false,
            key_u: false,
            key_v: false,
            key_w: false,
            key_x: false,
            key_y: false,
            key_z: false,
        }
    }

    /// When the Enter key is down(up), is_enter_down returns true(false).
    pub fn is_enter_down(&self) -> bool {
        self.enter
    }

    /// When the ArrowLeft key is down(up), is_arrow_left_down returns true(false).
    pub fn is_arrow_left_down(&self) -> bool {
        self.arrow_left
    }

    /// When the ArrowUp key is down(up), is_arrow_up_down returns true(false).
    pub fn is_arrow_up_down(&self) -> bool {
        self.arrow_up
    }

    /// When the ArrowRight key is down(up), is_arrow_right_down returns true(false).
    pub fn is_arrow_right_down(&self) -> bool {
        self.arrow_right
    }

    /// When the ArrowDown key is down(up), is_arrow_down_down returns true(false).
    pub fn is_arrow_down_down(&self) -> bool {
        self.arrow_down
    }

    /// When the Digit0 key is down(up), is_digit_0_down returns true(false).
    pub fn is_digit_0_down(&self) -> bool {
        self.digit_0
    }

    /// When the Digit1 key is down(up), is_digit_1_down returns true(false).
    pub fn is_digit_1_down(&self) -> bool {
        self.digit_1
    }

    /// When the Digit2 key is down(up), is_digit_2_down returns true(false).
    pub fn is_digit_2_down(&self) -> bool {
        self.digit_2
    }

    /// When the Digit3 key is down(up), is_digit_3_down returns true(false).
    pub fn is_digit_3_down(&self) -> bool {
        self.digit_3
    }

    /// When the Digit4 key is down(up), is_digit_4_down returns true(false).
    pub fn is_digit_4_down(&self) -> bool {
        self.digit_4
    }

    /// When the Digit5 key is down(up), is_digit_5_down returns true(false).
    pub fn is_digit_5_down(&self) -> bool {
        self.digit_5
    }

    /// When the Digit6 key is down(up), is_digit_6_down returns true(false).
    pub fn is_digit_6_down(&self) -> bool {
        self.digit_6
    }

    /// When the Digit7 key is down(up), is_digit_7_down returns true(false).
    pub fn is_digit_7_down(&self) -> bool {
        self.digit_7
    }

    /// When the Digit8 key is down(up), is_digit_8_down returns true(false).
    pub fn is_digit_8_down(&self) -> bool {
        self.digit_8
    }

    /// When the Digit9 key is down(up), is_digit_9_down returns true(false).
    pub fn is_digit_9_down(&self) -> bool {
        self.digit_9
    }

    /// When the KeyA key is down(up), is_key_a_down returns true(false).
    pub fn is_key_a_down(&self) -> bool {
        self.key_a
    }

    /// When the KeyB key is down(up), is_key_b_down returns true(false).
    pub fn is_key_b_down(&self) -> bool {
        self.key_b
    }

    /// When the KeyC key is down(up), is_key_c_down returns true(false).
    pub fn is_key_c_down(&self) -> bool {
        self.key_c
    }

    /// When the KeyD key is down(up), is_key_d_down returns true(false).
    pub fn is_key_d_down(&self) -> bool {
        self.key_d
    }

    /// When the KeyE key is down(up), is_key_e_down returns true(false).
    pub fn is_key_e_down(&self) -> bool {
        self.key_e
    }

    /// When the KeyF key is down(up), is_key_f_down returns true(false).
    pub fn is_key_f_down(&self) -> bool {
        self.key_f
    }

    /// When the KeyG key is down(up), is_key_g_down returns true(false).
    pub fn is_key_g_down(&self) -> bool {
        self.key_g
    }

    /// When the KeyH key is down(up), is_key_h_down returns true(false).
    pub fn is_key_h_down(&self) -> bool {
        self.key_h
    }

    /// When the KeyI key is down(up), is_key_i_down returns true(false).
    pub fn is_key_i_down(&self) -> bool {
        self.key_i
    }

    /// When the KeyJ key is down(up), is_key_j_down returns true(false).
    pub fn is_key_j_down(&self) -> bool {
        self.key_j
    }

    /// When the KeyK key is down(up), is_key_k_down returns true(false).
    pub fn is_key_k_down(&self) -> bool {
        self.key_k
    }

    /// When the KeyL key is down(up), is_key_l_down returns true(false).
    pub fn is_key_l_down(&self) -> bool {
        self.key_l
    }

    /// When the KeyM key is down(up), is_key_m_down returns true(false).
    pub fn is_key_m_down(&self) -> bool {
        self.key_m
    }

    /// When the KeyN key is down(up), is_key_n_down returns true(false).
    pub fn is_key_n_down(&self) -> bool {
        self.key_n
    }

    /// When the KeyO key is down(up), is_key_o_down returns true(false).
    pub fn is_key_o_down(&self) -> bool {
        self.key_o
    }

    /// When the KeyP key is down(up), is_key_p_down returns true(false).
    pub fn is_key_p_down(&self) -> bool {
        self.key_p
    }

    /// When the KeyQ key is down(up), is_key_q_down returns true(false).
    pub fn is_key_q_down(&self) -> bool {
        self.key_q
    }

    /// When the KeyR key is down(up), is_key_r_down returns true(false).
    pub fn is_key_r_down(&self) -> bool {
        self.key_r
    }

    /// When the KeyS key is down(up), is_key_s_down returns true(false).
    pub fn is_key_s_down(&self) -> bool {
        self.key_s
    }

    /// When the KeyT key is down(up), is_key_t_down returns true(false).
    pub fn is_key_t_down(&self) -> bool {
        self.key_t
    }

    /// When the KeyU key is down(up), is_key_u_down returns true(false).
    pub fn is_key_u_down(&self) -> bool {
        self.key_u
    }

    /// When the KeyV key is down(up), is_key_v_down returns true(false).
    pub fn is_key_v_down(&self) -> bool {
        self.key_v
    }

    /// When the KeyW key is down(up), is_key_w_down returns true(false).
    pub fn is_key_w_down(&self) -> bool {
        self.key_w
    }

    /// When the KeyX key is down(up), is_key_x_down returns true(false).
    pub fn is_key_x_down(&self) -> bool {
        self.key_x
    }

    /// When the KeyY key is down(up), is_key_y_down returns true(false).
    pub fn is_key_y_down(&self) -> bool {
        self.key_y
    }

    /// When the KeyZ key is down(up), is_key_z_down returns true(false).
    pub fn is_key_z_down(&self) -> bool {
        self.key_z
    }

    pub(crate) fn update_on_keydown(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            web_sys::KeyEvent::DOM_VK_RETURN => {
                self.enter = true;
            }
            web_sys::KeyEvent::DOM_VK_LEFT => {
                self.arrow_left = true;
            }
            web_sys::KeyEvent::DOM_VK_UP => {
                self.arrow_up = true;
            }
            web_sys::KeyEvent::DOM_VK_RIGHT => {
                self.arrow_right = true;
            }
            web_sys::KeyEvent::DOM_VK_DOWN => {
                self.arrow_down = true;
            }
            web_sys::KeyEvent::DOM_VK_0 => {
                self.digit_0 = true;
            }
            web_sys::KeyEvent::DOM_VK_1 => {
                self.digit_1 = true;
            }
            web_sys::KeyEvent::DOM_VK_2 => {
                self.digit_2 = true;
            }
            web_sys::KeyEvent::DOM_VK_3 => {
                self.digit_3 = true;
            }
            web_sys::KeyEvent::DOM_VK_4 => {
                self.digit_4 = true;
            }
            web_sys::KeyEvent::DOM_VK_5 => {
                self.digit_5 = true;
            }
            web_sys::KeyEvent::DOM_VK_6 => {
                self.digit_6 = true;
            }
            web_sys::KeyEvent::DOM_VK_7 => {
                self.digit_7 = true;
            }
            web_sys::KeyEvent::DOM_VK_8 => {
                self.digit_8 = true;
            }
            web_sys::KeyEvent::DOM_VK_9 => {
                self.digit_9 = true;
            }
            web_sys::KeyEvent::DOM_VK_A => {
                self.key_a = true;
            }
            web_sys::KeyEvent::DOM_VK_B => {
                self.key_b = true;
            }
            web_sys::KeyEvent::DOM_VK_C => {
                self.key_c = true;
            }
            web_sys::KeyEvent::DOM_VK_D => {
                self.key_d = true;
            }
            web_sys::KeyEvent::DOM_VK_E => {
                self.key_e = true;
            }
            web_sys::KeyEvent::DOM_VK_F => {
                self.key_f = true;
            }
            web_sys::KeyEvent::DOM_VK_G => {
                self.key_g = true;
            }
            web_sys::KeyEvent::DOM_VK_H => {
                self.key_h = true;
            }
            web_sys::KeyEvent::DOM_VK_I => {
                self.key_i = true;
            }
            web_sys::KeyEvent::DOM_VK_J => {
                self.key_j = true;
            }
            web_sys::KeyEvent::DOM_VK_K => {
                self.key_k = true;
            }
            web_sys::KeyEvent::DOM_VK_L => {
                self.key_l = true;
            }
            web_sys::KeyEvent::DOM_VK_M => {
                self.key_m = true;
            }
            web_sys::KeyEvent::DOM_VK_N => {
                self.key_n = true;
            }
            web_sys::KeyEvent::DOM_VK_O => {
                self.key_o = true;
            }
            web_sys::KeyEvent::DOM_VK_P => {
                self.key_p = true;
            }
            web_sys::KeyEvent::DOM_VK_Q => {
                self.key_q = true;
            }
            web_sys::KeyEvent::DOM_VK_R => {
                self.key_r = true;
            }
            web_sys::KeyEvent::DOM_VK_S => {
                self.key_s = true;
            }
            web_sys::KeyEvent::DOM_VK_T => {
                self.key_t = true;
            }
            web_sys::KeyEvent::DOM_VK_U => {
                self.key_u = true;
            }
            web_sys::KeyEvent::DOM_VK_V => {
                self.key_v = true;
            }
            web_sys::KeyEvent::DOM_VK_W => {
                self.key_w = true;
            }
            web_sys::KeyEvent::DOM_VK_X => {
                self.key_x = true;
            }
            web_sys::KeyEvent::DOM_VK_Y => {
                self.key_y = true;
            }
            web_sys::KeyEvent::DOM_VK_Z => {
                self.key_z = true;
            }
            _ => {}
        }
    }

    pub(crate) fn update_on_keyup(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            web_sys::KeyEvent::DOM_VK_RETURN => {
                self.enter = false;
            }
            web_sys::KeyEvent::DOM_VK_LEFT => {
                self.arrow_left = false;
            }
            web_sys::KeyEvent::DOM_VK_UP => {
                self.arrow_up = false;
            }
            web_sys::KeyEvent::DOM_VK_RIGHT => {
                self.arrow_right = false;
            }
            web_sys::KeyEvent::DOM_VK_DOWN => {
                self.arrow_down = false;
            }
            web_sys::KeyEvent::DOM_VK_0 => {
                self.digit_0 = false;
            }
            web_sys::KeyEvent::DOM_VK_1 => {
                self.digit_1 = false;
            }
            web_sys::KeyEvent::DOM_VK_2 => {
                self.digit_2 = false;
            }
            web_sys::KeyEvent::DOM_VK_3 => {
                self.digit_3 = false;
            }
            web_sys::KeyEvent::DOM_VK_4 => {
                self.digit_4 = false;
            }
            web_sys::KeyEvent::DOM_VK_5 => {
                self.digit_5 = false;
            }
            web_sys::KeyEvent::DOM_VK_6 => {
                self.digit_6 = false;
            }
            web_sys::KeyEvent::DOM_VK_7 => {
                self.digit_7 = false;
            }
            web_sys::KeyEvent::DOM_VK_8 => {
                self.digit_8 = false;
            }
            web_sys::KeyEvent::DOM_VK_9 => {
                self.digit_9 = false;
            }
            web_sys::KeyEvent::DOM_VK_A => {
                self.key_a = false;
            }
            web_sys::KeyEvent::DOM_VK_B => {
                self.key_b = false;
            }
            web_sys::KeyEvent::DOM_VK_C => {
                self.key_c = false;
            }
            web_sys::KeyEvent::DOM_VK_D => {
                self.key_d = false;
            }
            web_sys::KeyEvent::DOM_VK_E => {
                self.key_e = false;
            }
            web_sys::KeyEvent::DOM_VK_F => {
                self.key_f = false;
            }
            web_sys::KeyEvent::DOM_VK_G => {
                self.key_g = false;
            }
            web_sys::KeyEvent::DOM_VK_H => {
                self.key_h = false;
            }
            web_sys::KeyEvent::DOM_VK_I => {
                self.key_i = false;
            }
            web_sys::KeyEvent::DOM_VK_J => {
                self.key_j = false;
            }
            web_sys::KeyEvent::DOM_VK_K => {
                self.key_k = false;
            }
            web_sys::KeyEvent::DOM_VK_L => {
                self.key_l = false;
            }
            web_sys::KeyEvent::DOM_VK_M => {
                self.key_m = false;
            }
            web_sys::KeyEvent::DOM_VK_N => {
                self.key_n = false;
            }
            web_sys::KeyEvent::DOM_VK_O => {
                self.key_o = false;
            }
            web_sys::KeyEvent::DOM_VK_P => {
                self.key_p = false;
            }
            web_sys::KeyEvent::DOM_VK_Q => {
                self.key_q = false;
            }
            web_sys::KeyEvent::DOM_VK_R => {
                self.key_r = false;
            }
            web_sys::KeyEvent::DOM_VK_S => {
                self.key_s = false;
            }
            web_sys::KeyEvent::DOM_VK_T => {
                self.key_t = false;
            }
            web_sys::KeyEvent::DOM_VK_U => {
                self.key_u = false;
            }
            web_sys::KeyEvent::DOM_VK_V => {
                self.key_v = false;
            }
            web_sys::KeyEvent::DOM_VK_W => {
                self.key_w = false;
            }
            web_sys::KeyEvent::DOM_VK_X => {
                self.key_x = false;
            }
            web_sys::KeyEvent::DOM_VK_Y => {
                self.key_y = false;
            }
            web_sys::KeyEvent::DOM_VK_Z => {
                self.key_z = false;
            }
            _ => {}
        }
    }
}
