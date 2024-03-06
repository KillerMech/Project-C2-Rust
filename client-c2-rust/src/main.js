import { createApp } from 'vue'
import App from './App.vue'

import PrimeVue from 'primevue/config'
import Card from 'primevue/card'
import Button from 'primevue/button'
import PanelMenu from 'primevue/panelmenu'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Toast from 'primevue/toast'
import ToastService from 'primevue/toastservice'

import "primevue/resources/themes/md-dark-indigo/theme.css"
import "primevue/resources/primevue.min.css"
import "primeicons/primeicons.css"

createApp(App)
    .use(PrimeVue)
    .use(ToastService)

    .component('Button', Button)
    .component('Card', Card)
    .component('PanelMenu', PanelMenu)
    .component('Dialog', Dialog)
    .component('Button', Button)
    .component('InputText', InputText)
    .component('Toast', Toast)

    .mount('#app')

