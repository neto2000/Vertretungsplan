<script>

  import { show_add_mask } from "../stores";


  let teacher = "";

  let fach = "";

  let fach_kuerzel = "";

  let room = "";



  function quit() {

    show_add_mask.set(false);
  }

  function add() {
 
    
    show_add_mask.set(false);

  }


</script>


 <!--div class="mask-container"-->
  <div class="mask-background">
    <h1 class="add-heading">Stunde hinzufügen</h1>

    <div class="add-grid">

      <div class="icon-container">
        <img src="/images/clock.svg" alt="Clock" class="mask-icon">
      </div>

      <div class="selector-container">
        <select name="stunde1" id="stunde1" class="select">

          {#each {length: 10} as _, i}

            <option value={i + 1}>{i + 1}. Stunde</option>

          {/each}
        </select>

        -

        <select name="stunde2" id="stunde2" class="select" style="margin-left: 10px;">

          {#each {length: 10} as _, i}

            <option value={i + 1}>{i + 1}. Stunde</option>

          {/each}
        </select>

      </div>

      

      <div class="icon-container">

        <img src="/images/person2.svg" alt="Teacher" class="mask-icon">
      </div>

      <div class="selector-container">
        
        <select name="anrede" id="anrede" class="select">

          <option value="Hr.">Hr.</option>
          <option value="Fr.">Fr.</option>

        </select>


        <input type="text" placeholder="Name des Lehrers" class="input" bind:value={teacher}>

      </div>


      
      <div class="icon-container">

        <img src="/images/book.svg" alt="Fach" class="mask-icon">
      </div>

      <div class="selector-container">

        <input type="text" placeholder="Fach" class="input" style="width: 100px; margin-right: 10px;" bind:value={fach}>
        <input type="text" placeholder="Kürzel" class="input" style="width: 50px;" bind:value={fach_kuerzel}>

      </div>



      <div class="icon-container">

        <img src="/images/door.svg" alt="Room" class="mask-icon">
      </div>

      <div class="selector-container">

        <input type="text" placeholder="Raum" class="input" style="width: 38px;" bind:value={room}>

      </div>

    </div>

    <div class="button-container">

      <button class="quit-button" on:click={quit}>Abbrechen</button>
      <button class="add-button" on:click={add}>Hinzufügen</button>

    </div>
  </div>
<!--/div-->



<style>

  .mask-container {

    position: absolute;

    display: flex;

    top: 0;

    left: 0;

    justify-content: center;
  
    align-items: center;

    width: 100%;
    height: 100vh;


  }

  .mask-background {

    position: fixed;

    background-color: white;

    z-index: 10;


    height: 650px;
    width: 800px;


    border: none;

    border-radius: 26px;

    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);


    text-align: center;

    vertical-align: middle;


    top: 50%;

    left: 50%;


    transform: translate(-50%,-50%);


  }

  .add-heading {

    font-family: 'Poppins', sans-serif;

    font-size: 40px;

  }


  .add-grid {

    display: grid;

    grid-template-columns: 25% 75%;

    row-gap: 50px;

    align-items: center;


    margin-top: 70px; 

  }


  .button-container {

    display: flex;

    justify-content: right;

    align-items:flex-end;

    height: 145px;

  }

  .selector-container {

    display: flex; 
    
    align-items: center;

  }

  .icon-container {

    display: flex; 
    
    align-items: center;

    justify-content: center;
  }

  .input {

    padding: 5px 10px 7px 10px;


    border-radius: 8px;  

    border-style: solid;

    border-width: 1px;

    border-color: black;

  }

  .select {


    padding: 5px 3px 7px 10px;

    margin-right: 10px;


    border-radius: 8px;  

    border-style: solid;

    border-width: 1px;

    border-color: black;


    background-color: white;

  }

  .mask-icon {

    width: 40px;

    height: 40px;

  }

  .quit-button {

    background-color: white;

    
    padding: 7px 30px 9px 30px;


    border: 1px solid black;

    border-radius: 8px;

  }

  .add-button {

    background-color: var(--accent);

    color: white;

    padding: 7px 30px 9px 30px;


    border: 1px solid var(--accent);

    border-radius: 8px;


    margin-left: 15px;

    margin-right: 40px;

  }

</style>
