<script>
  import { show_add_mask } from "../stores";

  import StundenplanTabelle from "./Stundenplan_Tabelle.svelte";

  import AddMask from "./Add_Mask.svelte";


  let show_mask;

  show_add_mask.subscribe((value) => {

    show_mask = value;
  });
  


  let is_edit = false

  function edit() {


    is_edit = !is_edit;

  }

</script>

{#if show_mask} 
  <AddMask/>
{/if}


<section class="stundenplan-container">
  <h1 class="heading">Dein Stundenplan</h1>

  <StundenplanTabelle is_edit={is_edit}/>

  <div class="edit-container">
    <button class="edit-button" on:click={edit}>

      {#if is_edit}
        <img src="/src/assets/add.svg" alt="Edit" class="cancel-image"> 
      {:else}
         
        <img src="/src/assets/edit.svg" alt="Edit" class="edit-image"> 

      {/if}

    </button>
  </div>


</section>


<style>


  .stundenplan-container {

    margin-top: 10vh;

    background-color: var(--secondary);

    width: 83%;

    position: relative;

    left: 50%;

    transform: translateX(-50%);

    
    border: none;

    border-radius: 29px;


    display: flex;


    align-items: center;

    flex-direction: column;

  }

  .heading {
    font-family: 'Poppins', sans-serif;

    font-size: 40px;

    color: var(--primary);
  }


  .edit-container {

    margin-top: 60px;


    width: 100%;

    display: flex;

    justify-content: right;
  }

  .edit-image {

    width: 50px;

    
  }

  .cancel-image {

    width: 60px;

    transform: rotate(45deg);
  }

  .edit-button {

    border: none;

    border-radius: 15px;

    background-color: var(--accent);

    height: 60px;
    width: 60px;


    margin-bottom: 40px;
    margin-right: 55px;

    cursor: pointer;  


    display: flex;

    align-items: center;
    justify-content: center;

  }

</style>
