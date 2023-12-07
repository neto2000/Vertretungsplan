<script>
  export let logged_in = true;

  function test() {

    console.log("Main Page")

  }
</script>

<header class="header">

  <div class="header-container">

    <div class="link-container">
      <button class="header-link">Stundenplan</button>
      <button class="header-link">Aushänge</button>
      <button class="header-link">Über uns</button>
    </div>

    <div class="logo-container">
      <button class="logo" on:click={test}>VP</button>
    </div>

    <div class="account">
      {#if logged_in}
        <p class="user-name">neto2000</p>
        <button class="settings">
          <img class="user-pic" src="./src/assets/account_circle.svg" alt="account" />
        </button>
      {:else}
        <button class="login">Log In</button>
        <button class="signup">Sign Up</button>
      {/if} 
    </div>

  </div>

</header>

<style>

  .header {
    
    width: 100%;

    height: 8.5vh;

    border-style: none none solid none;

    border-width: thin;

    border-color: #DDDDDD;


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
  }

  .logo {

    background-color: white;

    border: none;

    font-family: 'Megrim', sans-serif;

    font-size: 40px;


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
