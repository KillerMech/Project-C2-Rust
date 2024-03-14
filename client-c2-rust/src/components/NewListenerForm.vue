<template>
    <div>
        <h2>New Listener Form</h2>
        <div class="card flex justify-content-center">
            <label for="type" style="font-weight: bold;">Type</label>
            <Dropdown id="type" v-model="type" :options="listenerTypes" optionLabel="name" placeholder="Not yet implemented"
                class="w-flex md:w-20rem"/>
            <small id="type-help"><br>Type of listener (not yet implemented)</small>
        </div>
        <div class="flex flex-column gap-2">
            <label for="port">Port</label>
            <InputText id="port" v-model="port" class="w-full md:w-20rem"/>
            <small id="port-help">Port to listen on</small>
        </div>
        <Button label="Submit" @click="submit" />
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import axios from 'axios';

    const type = ref('');
    const port = ref('');
    const listenerTypes = ref([
        {name: 'http'}, 
        {name: 'https'}
        ]);

    const submit = async () => {
       try {
           // Todo: Get rid of console log
           console.log("http://localhost:"+window.location.port+'/new_listener');
           const response = await axios.post('http://localhost:'+window.location.port+'/new_listener', {
               type: type.value,
               port: port.value
           });
           console.log(response);
       } catch (error) {
           console.error(error);
       }
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
