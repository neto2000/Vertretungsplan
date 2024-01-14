import { writable } from 'svelte/store';

export const show_add_mask = writable(false);

export const is_admin = writable(true);

export const Pages = {
        Main: 'Main',
        Stundenplan: 'Stundenplan',
        About: 'About',
        Anhaenge: 'Anhaenge',
        Login: 'Login',
        SignUp: 'SignUp',
        Admin: 'Admin',
}

export const current_page = writable(Pages.Main);





