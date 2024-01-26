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


export const admin_rows = writable([{id:0, class:"7b", hour:"5-6", fach_old:"D", away:"Mustermann", sub:"Horst", fach_new:"D", room:"400", typ:"Vertretung", info:"-"}]);

export const changed_rows = writable([]);


