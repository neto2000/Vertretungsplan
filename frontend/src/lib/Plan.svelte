<script>
  import Info from "./Info.svelte";
import Table from "./Table.svelte";
</script>


<section>
  <div class="plan-container">
    <h2 class="plan-heading">Montag der 08.09.2023</h2>

    <div class="button-container">
      <button class="active">Heute</button>
      <button class="not-active">Morgen</button>
    </div>

    <div class="info-container">
      <h3 class="info-head">Infos</h3>

      <div class="infos">
        {#each Array(4) as _, i}
          <Info text={"Das ist eine wichtige Info!"} />
        {/each}
      </div>

    </div>

    <Table />


  </div>
</section>


<style>
  .plan-container {


    margin-top: 10vh;



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

    color: #003a6c;

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 40px;

  }


  .active {

    padding-top: 8px;
    padding-bottom: 8px;

    width: 110px;

    background-color: #1c254a;
  

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: #e2edf6;

    border: none;
    border-radius: 10px;

  }

  .not-active {

    padding-top: 7px;
    padding-bottom: 7px;

    background-color: #e2edf6;

    width: 110px;
    
    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: #1c254a;

    border: solid;
    border-color: #1c254a;
    border-width: thin;
    border-radius: 10px;

  }

  .info-container {

    background-color: white;
  

    width: 80%;

    
    border: none;
    border-radius: 10px;

  }

  .info-head {

    color: #000000;

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 32px;


  }
</style>
