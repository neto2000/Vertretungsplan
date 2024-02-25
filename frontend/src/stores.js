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


export const admin_rows = writable([{id:0, class:"7b", start_hour:5, end_hour: 6, old_fach:"D", away:"Mustermann", sub:"Horst", new_fach:"D", room:"400", typ:"Vertretung", info:"-"}]);

export const changed_rows = writable([]);


