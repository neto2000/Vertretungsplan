<script>

  import { current_page, Pages } from "../stores";


  let show_alert1 = false;

  let show_alert2 = false;

  function change_page(page) {
    
    current_page.set(page);

  }
</script>


<div class="login-site">
  <div class="login-container">


    <div style="width: 100%;">

      <button class="back" on:click={() => {change_page(Pages.Main)}}>
        <img class="back-icon" src="/images/back-arrow.svg" alt="back">
      </button>

    </div>

    <div class="mid-section">


      <input type="text" class="input-box" placeholder="Enter your Username">

      <input type="text" class="input-box" placeholder="Enter your E-Mail">
      

      {#if show_alert1} 
        <p class="alert-text" style="margin-right: 280px;">invalid email</p>
      {:else} 
        <p class="alert-text" style="color: white;">nothing to see here</p>
      {/if}

      <input type="password" class="input-box" placeholder="Enter your password">

      <input type="password" class="input-box" placeholder="Retype your password">

      {#if show_alert2} 
        <p class="alert-text">passwords do not match</p>
      {:else} 
        <p class="alert-text" style="color: white;">nothing to see here</p>
      {/if}


      <button class="login-button">Register</button>

      <p class="trenner-text">- or -</p>

      <button class="google-button">Continue with Google</button>
    </div>

  </div>
</div>


<style>
  .login-site{

    display: flex;

    justify-content: center;

    align-items: center;

    height: 100vh;
  }

  .login-container {

    display: flex;

    flex-direction: column;

    align-items: center;


    width: 550px;

    height: 60vh;

    background-color: white;


    border: none;

    border-radius: 20px;

    
    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);
  }

  .mid-section {

    display: flex;

    justify-content: center;

    align-items: center;

    flex-direction: column;

    width: 100%;



    margin-top: 30px;

  }

  .input-box {

    font-family: rubik, 'sans-serif';
    font-size: 16px;

    width: 350px;

    height: 30px;


    border-radius: 5px;

    border: 1px solid grey;


    padding-left: 10px;

    margin-bottom: 10px;

    margin-top: 15px;

  }


  .login-button {

    font-family: rubik, 'sans-serif';
    font-size: 16px;

    border: none;

    border-radius: 10px;

    background-color: var(--accent);

    color: white;


    width: 250px;

    height: 35px;



    margin-top: 25px;

  }

  .trenner-text {

    font-family: rubik, 'sans-serif';


    font-size: 16px;

  }

  .google-button {

    font-family: rubik, 'sans-serif';
    font-size: 13px;

    border: 1px solid black;

    border-radius: 10px;

    background-color: white;



    width: 250px;

    height: 30px;

  }


  .alert-text {
    
    color: red;

    font-family: rubik, 'sans-serif';
    font-size: 11px;

    margin: 0;

    margin-right: 220px;

  }

  .heading {

    font-family: 'Poppins', sans-serif;

    font-size: 40px;


    margin: 0;
  }

  .back {

    border: none;

    background-color: white;

    margin-top: 10px;

    margin-left: 4px;

  }

  .back-icon  {
  
    width: 50px;

  }
</style>
