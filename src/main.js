import Vue from "vue"
import App from "./App.vue"

import {initContract} from "./utils"
import {BootstrapVue} from "bootstrap-vue"

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

Vue.config.productionTip = false
Vue.use(BootstrapVue)

window.nearInitPromise = initContract()
  .then(() => {

    new Vue({
      render: h => h(App),
    }).$mount("#app")

  })
