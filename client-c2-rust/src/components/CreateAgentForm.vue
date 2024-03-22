<template>
    <div>
        <h2>Create New Agent</h2>
        <div class="card flex justify-content-center">
            <label for="ip_address" style="font-weight: bold;">IP Address</label>
            <InputText id="ip_address" v-model="ip_address"  optionLabel="name" class="w-flex md:w-20rem"/>
            <br>
            <small id="type-help"><br>IP Address for callback</small>
        </div>
        <div class="flex flex-column gap-2">
            <label for="port">Port</label>
            <InputText id="port" v-model="port" class="w-full md:w-20rem"/>
            <br>
            <small id="port-help">Port to listen on</small>
        </div>
        <Button label="Submit" @click="submit" />
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import axios from 'axios';

    const ipAddy = ref('');
    const port = ref('');

    const submit = async () => {
       try {
           // Todo: Get rid of console log
           console.log("http://localhost:"+window.location.port+'/create_agent');
           const response = await axios.post('http://localhost:'+window.location.port+'/create_agent', {
               ip_addy: ipAddy.value,
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
