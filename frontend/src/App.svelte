<script>


    import { current_page, Pages } from "./stores";

    import Header from './lib/Header.svelte'
    import Plan from './lib/Plan.svelte'
    import Personal from './lib/Personal.svelte';
    import Stundenplan from './lib/Stundenplan.svelte';
    import Login from "./lib/Login.svelte";

    let current;

    current_page.subscribe((value) => {
        current = value;
    })


</script>

<main>

    {#if current == Pages.Main}
         
        <Header  />

        <Personal />
        <Plan />

    {:else if  current == Pages.Stundenplan}

        <Header  />

        <Stundenplan  />
    {:else if current == Pages.Login}

        <Login />

    {/if}



    
</main>

<style>
</style>
