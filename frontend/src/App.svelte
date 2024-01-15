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

    current_page.subscribe((value) => {
        current = value;
    })


</script>

<main>

    {#if current == Pages.Main}
         
        <Header  />
        
        <div class="header-space">

            <Personal />
            <Plan />

        </div>
    {:else if  current == Pages.Stundenplan}

        <Header  />

        <div class="header-space">

            <Stundenplan  />

        </div>
    {:else if current == Pages.Login}

        <Login />

    {:else if current == Pages.SignUp}
        
        <SignUp />

    {:else if current == Pages.Admin}

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
