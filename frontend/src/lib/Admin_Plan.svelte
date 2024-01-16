<script>
  import AdminTabelle from "./Admin_Tabelle.svelte";
</script>


<section>
  <div class="plan-container">
    <h2 class="plan-heading">Vertretungsplan</h2>

    <div class="button-container">
      <button class="active">&lt;</button>
      <button class="not-active">Montag, der 17.01.2024</button>
      <button class="active">&gt;</button>
    </div>

    <div class="info-container">
      <h3 class="info-head">Infos</h3>

      <div class="infos">
        
      </div>

    </div>

    <AdminTabelle />


  </div>

</section>


<style>
  .plan-container {


    margin-top: 10vh;
    margin-bottom: 20vh;


    background-color: #e2edf6;

    border-style: none;
    border-radius: 29px;

    width: 78.5%;

    position: relative;

    left: 50%;

    transform: translate(-50%);


    display: flex;

    flex-direction: column;

    align-items: center;

  }

  .plan-heading {

    color: var(--primary);

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 40px;

    margin: 6vh 0vh 5vh 0vh;

  }


  .active {

    padding-top: 8px;
    padding-bottom: 8px;

    margin-right: 5px;

    width: 110px;

    background-color: var(--accent);
  

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--secondary);

    border: none;
    border-radius: 10px;


     cursor: pointer;

  }

  .not-active {

    padding-top: 7px;
    padding-bottom: 7px;

    margin-left: 5px;

    background-color: var(--secondary);

    width: 110px;
    
    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--accent);

    border: solid;
    border-color: var(--accent);
    border-width: thin;
    border-radius: 10px;

    
    cursor: pointer;
  }

  .not-active:hover {

    opacity: 70%;

  }

  .info-container {

    background-color: white;
  

    width: 60%;


    margin-top: 9vh;

    padding: 4vh 0vh;

    
    border: none;
    border-radius: 10px;

    box-shadow: 0px 6px 6px 4px rgba(0, 0, 0, 0.2);

  }

  .info-head {

    color: #000000;

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 32px;

    margin: 0vh 0vh 0vh 6vh;


  }

  .infos {
    
    margin-top: 2vh

  }
</style>
