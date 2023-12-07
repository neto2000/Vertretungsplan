<script>

</script>


<section class="stundenplan-container">
  <h1 class="heading">Dein Stundenplan</h1>

  <div class="tabelle">
    
  </div>

  <div class="edit-container">
    <button class="edit-button">
      <img src="/src/assets/edit.svg" alt="Edit" class="edit-image"> 
    </button>
  </div>


</section>


<style>


  .stundenplan-container {

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

    width: 100%;

    display: flex;

    justify-content: right;
  }

  .edit-image {

    width: 45px;

    
  }

  .edit-button {

    border: none;

    border-radius: 15px;

    background-color: var(--accent);

    height: 60px;
    width: 60px;


    margin-bottom: 40px;
    margin-right: 55px;



    display: flex;

    align-items: center;
    justify-content: center;

  }

</style>
