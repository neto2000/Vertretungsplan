<script>

  import { is_admin, current_page, Pages } from "../stores";
  import { fly } from 'svelte/transition';

  export let logged_in = false;


  
  let show_header = true;

  

  let y = 0;


  let last_y = 0;
  
  function get_direction(y) {

    let dy = y - last_y;

    last_y = y;


    if (dy > 0) {

      return false;
    }
    else {

      return true;
    }

  }


  $: show_header = get_direction(y)


  function change_page(page) {
    
    current_page.set(page);

  }

  let l_is_admin;
  
  is_admin.subscribe((value) => {
    l_is_admin = value;
  })
</script>


{#if show_header}


<header class="header" transition:fly={{ delay: 10, duration: 300, x: 0, y: -500, opacity: 0.5}}>

  <div class="header-container">

    <div class="link-container">
      <button class="header-link"><a href="/#stundenplan" class="header-a">Stundenplan</a></button>
      <button class="header-link">Aushänge</button>

      {#if l_is_admin}
         
        <button class="header-link"><a href="/#admin" class="header-a">Admin</a></button>
      {/if}

      <button class="header-link"><a href="/#login" class="header-a">Über uns</a></button>
    </div>

    <div class="logo-container">
      <button class="logo"><a href="/" class="header-a">VP</a></button>
    </div>

    <div class="account">
      {#if logged_in}
        <p class="user-name">neto2000</p>
        <button class="settings">
          <img class="user-pic" src="/images/account_circle.svg" alt="account" />
        </button>
      {:else}
        <button class="login"><a href="/#login" class="header-a">Log In</a></button>
        <button class="signup"><a href="/#signup" class="header-a" style="color: white;">Sign Up</a></button>
      {/if} 
    </div>

  </div>

</header>

{/if}


<svelte:window bind:scrollY={y} />


<style>

  .header {
    
    width: 100%;

    height: 8.5vh;

    border-style: none none solid none;

    border-width: thin;

    border-color: #DDDDDD;
  
    background-color: #FFFFFF;  


    position: fixed;

    z-index: 5;
  }

  .header-container {

    height: 100%;

    display: grid;

    grid-template-columns: 45% 10% 45%;

  }

  .link-container {
    text-align: left;
  
    margin: auto 0;

  }

  .logo-container {

    text-align: center;

    
    margin: auto 0;


  }

  .account {

    text-align: right;

    height: 8.5vh;

    line-height: 8.5vh;
  }


  .header-link {

    margin-left: 3.5%;

    background-color: white;

    
    border: none;

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    cursor: pointer;

  }

  .header-link:hover {

    opacity: 60%;

  }

  .header-a {
    color: black;

    text-decoration: none;

  }

  .logo {

    background-color: white;

    border: none;

    font-family: 'Megrim', sans-serif;

    font-size: 40px;

    cursor: pointer;
  }

  .signup {

    margin-left: 1.7%;
    margin-right: 3.5%;

    padding: 7px 15px;

    border: none; 
    border-radius: 10px;

    background-color: var(--primary);
    
    color: #DBE6F0;  


    font-family: 'Rubik', sans-serif;

    font-size: 16px;
  }

  .login {

    padding: 6px 15px;

    border: solid;
    border-width: thin;
    border-color: #575757;
    border-radius: 10px;

    background-color: white;
      


    font-family: 'Rubik', sans-serif;

    font-size: 16px;

  }

  .user-name {
    
    font-family: 'Rubik', sans-serif;
    
    font-size: 16px;

    display: inline-block;

    vertical-align: middle;

    line-height: normal;

    margin-right: 1.7%;

  }

  .user-pic {

    display: inline-block;

    vertical-align: middle;

    line-height: normal;

    width: 50px;

    color: var(--secondary);

  }

  .settings {

    border: none;

    background: white;

    margin-right: 3.5%;

    width: 50px;

    height: 50px;

    padding: 0;

  }

  
  
 


</style>
