<template>
  <main>
    <header class="pt-3 mb-4">
      <div class="container d-flex justify-content-between">
        <span>{{ accountId }}</span>
        <button class="btn btn-primary" v-on:click="logout">Sign out</button>
      </div>
    </header>

    <div class="container">
      <h1 class="text-center">My Vouchers</h1>

      <b-row class="text-center mt-3">
        <b-col></b-col>
        <b-col lg="3" md="5">
          <form @submit.prevent="addVoucher">
            <b-input-group>
              <b-form-input type="number" required min="0.1" max="10" step="0.1" v-model="voucherDeposit"></b-form-input>
              <b-input-group-append>
                <b-button variant="primary" type="submit">Add Voucher</b-button>
              </b-input-group-append>
            </b-input-group>
          </form>
        </b-col>
        <b-col></b-col>
      </b-row>

      <hr>

      <div v-if="is_ready">
        <div v-if="vouchers.length" class="vouchers">
          <div v-for="voucher in vouchers" :key="voucher.id" class="one-voucher" :class="{'is-used':voucher.used_by}">
            <span class="used-label" v-if="voucher.used_by">USED by {{ voucher.used_by }}</span>
            <qrcode-vue :value="getUrl(voucher.id)" :size="300" level="H"/>
            <input type="hidden" :id="'clone-'+voucher.id" readonly :value="getUrl(voucher.id)"/>

            <p class="text-center">
              <img src="../assets/delete.png"
                   alt="remove"
                   title="remove"
                   class="small-button remove-voucher"
                   @click="removeVoucher(voucher.id)">
              {{ toNearAmount(voucher.deposit_amount) }} NEAR
              <img src="../assets/copy.png"
                   alt="copy"
                   title="copy"
                   class="small-button copy-link"
                   @click="copyURL(voucher.id)">
            </p>
          </div>
        </div>
        <div v-if="!vouchers.length" class="text-center">
          *No vouchers
        </div>
      </div>
      <div v-if="!is_ready" class="text-center">
        <img src="../assets/loader.gif" alt="" width="70">
      </div>
    </div>
  </main>
</template>

<script>
import {logout} from "../utils";
import Big from "big.js";
import sha256 from 'crypto-js/sha256';
import QrcodeVue from 'qrcode.vue';

export default {
  name: "Vouchers",
  beforeMount() {
    if (this.isSignedIn) {
      if (!localStorage.getItem('app-private-keys')) {
        localStorage.setItem('app-private-keys', JSON.stringify({}));
      }

      this.getVouchers();
    }
  },
  components: {
    QrcodeVue,
  },
  data: function () {
    return {
      is_ready: false,
      voucherDeposit: null,
      vouchers: [],
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract ? window.contract.contractId : null
    },
    networkId() {
      return window.networkId
    },
  },
  methods: {
    getVouchers() {
      this.is_ready = false;
      window.contract.user_vouchers({
        'account_id': window.accountId
      }).then(vouchers => {
        this.vouchers = [];
        vouchers.forEach(voucher => {
          if (this.isValidVoucher(voucher.id)) {
            this.vouchers.push(voucher);
          }
        });
        this.is_ready = true;
      })
    },

    async addVoucher() {
      if (this.voucherDeposit > 0) {
        let keys = JSON.parse(localStorage.getItem('app-private-keys'));
        const newId = this.randomStr(12);
        keys[newId] = this.randomStr(64);
        localStorage.setItem('app-private-keys', JSON.stringify(keys));

        const hash = sha256(keys[newId]).toString();
        const GAS = Big(300).times(10 ** 12).toFixed();
        const DEPOSIT = Big(this.voucherDeposit).times(10 ** 24).toFixed();
        try {
          await window.contract.add_voucher({
            id: newId,
            hash: hash,
          }, GAS, DEPOSIT);
        } catch (e) {
          alert("Something went wrong!");
          throw e //re-throw
        } finally {
          console.log('added');
          this.getVouchers();
        }
      } else {
        alert("Please specify deposit");
      }
    },

    async removeVoucher(id) {
      try {
        await window.contract.remove_voucher({
          id,
        });
      } catch (e) {
        alert("Something went wrong!");
        throw e //re-throw
      } finally {
        console.log('removed');
        this.getVouchers();
      }
    },

    getUrl(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return window.location.origin + `/?user=${window.accountId}&id=${id}#${keys[id]}`;
    },

    isValidVoucher(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return keys[id] !== undefined;
    },

    toNearAmount(value) {
      return Big(value).div(10 ** 24).toFixed();
    },

    copyURL(id) {
      const link = document.querySelector(`#clone-${id}`);
      link.setAttribute('type', 'text');
      link.select();
      document.execCommand("copy");
      link.setAttribute('type', 'hidden');
    },

    randomStr(length) {
      let result = '';
      const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
      const charactersLength = characters.length;
      for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
      }
      return result;
    },

    logout: logout,
  },
}
</script>
