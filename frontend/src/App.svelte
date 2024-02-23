<script>


    import { current_page, Pages } from "./stores";

    import Header from './lib/Header.svelte'
    import Plan from './lib/Plan.svelte'
    import Personal from './lib/Personal.svelte';
    import Stundenplan from './lib/Stundenplan.svelte';
    import Login from "./lib/Login.svelte";
    import SignUp from "./lib/SignUp.svelte";
    import Admin from "./lib/Admin.svelte";

    let current;

    let page = "main";

    current_page.subscribe((value) => {
        current = value;
    })


    const routing_map = {
        '#admin': "admin",
        '#login': "login",
        '#signup': "signup",
        '#stundenplan': "stundenplan",
        '': "main",
    };


    function routeChange() {
        page = routing_map[location.hash] || "not-found";

        console.log("hash change");
    }


</script>

<svelte:window on:hashchange={routeChange} />

<main>

    {#if page == "main"}
         
        <Header  />
        
        <div class="header-space">

            <Personal />
            <Plan />

        </div>
    {:else if page == "stundenplan"}

        <Header  />

        <div class="header-space">

            <Stundenplan  />

        </div>
    {:else if page == "login"}

        <Login />

    {:else if page == "signup"}
        
        <SignUp />

    {:else if page == "admin"}

        <Header  />

        <div class="header-space">
            
            <Admin /> 

        </div>
    {/if}

    
    
</main>

<style>

    .header-space {
    
      padding-top: 8.5vh;  

    }

</style>
