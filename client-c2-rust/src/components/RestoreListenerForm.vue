<template>
    <div>
        <h2>Attempted to restore listeners. Check server output.</h2>
    </div>
</template>

<script setup>
    import axios from 'axios';

   try {
       // Todo: Get rid of console log
       console.log("http://localhost:"+window.location.port+'/restore_listener');
       const response = await axios.post('http://localhost:'+window.location.port+'/restore_listener', {
       });
       console.log(response);
   } catch (error) {
       console.error(error);
   }
</script>

<style scoped>
.input-field > *{
    display: flex;
    flex-direction: column;
}
#port {
    margin-left: 1rem;
    width: 20rem;
}
#type {
    margin-left: 1rem;
    width: 20rem;
}
</style>
