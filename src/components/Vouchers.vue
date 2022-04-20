<template>
  <main>
    <Header></Header>

    <div class="container-lg position-relative">
      <button class="position-absolute rounded-circle border border-secondary print-btn" @click="print()">
        <img src="../assets/print.svg" width="18" alt="">
      </button>
      <h2 class="text-center mb-5 page-title">My Payment Vouchers</h2>

      <div v-if="is_ready">
        <p v-if="!vouchers.length" class="text-center">
          <b>You don't have vouchers, please create new one:</b>
        </p>
        <div :class="{'vouchers': vouchers.length, 'no-vouchers': !vouchers.length}">
          <div v-for="voucher in vouchers" :key="voucher.id" class="one-voucher" :class="{
            'is-used': voucher.used_by,
            'is-expired': isExpired(voucher.expire_date),
          }">
            <div class="additional-border"></div>
            <div class="voucher-inner">
              <div class="canvas text-center">
                <qrcode-vue :value="getUrl(voucher.id)" :size="220" level="H" />
              </div>
              <div class="gift-shadow"></div>
              <div class="gift-vertical">
                <img src="../assets/bn-1.svg" alt="">
              </div>
              <div class="black-bg">
                <input type="hidden" :id="'clone-'+voucher.id" readonly :value="getUrl(voucher.id)" />
                <img src="../assets/delete.png"
                     alt="remove"
                     title="remove"
                     class="small-button remove-voucher"
                     @click="removeVoucher(voucher.id)">

                <h4 class="text-center near-amount">
                  <span :class="{'is-used': voucher.used_by && voucher.payment_type === 'static'}">
                    {{ toNearAmount(voucher.deposit_amount) }} NEAR
                  </span>
                  <small class="used-label" v-if="!voucher.used_by && voucher.expire_date">
                    <small v-if="voucher.payment_type === 'static'">Expire<u class="text-decoration-none" v-if="isExpired(voucher.expire_date)">d</u>:</small>
                    <small v-if="voucher.payment_type === 'linear'">Full Unlock:</small>
                    {{ dateFormat(voucher.expire_date) }}
                  </small>

                  <small class="used-label" v-if="voucher.used_by">
                    <small>Unlocked by: {{ voucher.used_by }}</small>
                  </small>
                  <small class="used-label" v-if="voucher.paid_amount">
                    <b v-if="voucher.payment_type === 'static'">Claimed</b>
                    <b v-if="voucher.payment_type !== 'static'">Claimed: <br>
                      {{ parseFloat(toNearAmount(voucher.paid_amount)).toFixed(5) }} NEAR
                    </b>
                  </small>
                </h4>

                <div class="copy" @click="copyURL(voucher.id)">
                  <img src="../assets/copy.png"
                       alt="copy"
                       title="copy">
                  Copy URL
                </div>
              </div>
            </div>
          </div>

          <div class="one-voucher add-voucher-form">
            <form @submit.prevent="addVoucher" class="row">
              <div class="offset-1 col-10">
                <div class="additional-border"></div>
                <h4 class="text-center text-uppercase mb-3">
                  Create
                  <input type="text" maxlength="2" placeholder="1"
                         v-model="createCount" class="input-counter" />
                  Voucher{{ createCount > 1 ? "s" : "" }}
                </h4>
                <div class="row mb-2">
                  <div class="col-5 text-right">
                    <label class="form-check-label text-right pt-1">Payment Type<sup>*</sup></label>
                  </div>
                  <div class="col-6">
                    <b-form-select v-model="voucherType" :options="voucherTypeOptions"></b-form-select>
                  </div>
                </div>
                <div class="row mb-2">
                  <div class="col-5 text-right">
                    <label class="form-check-label pt-2">
                      {{ voucherType === 'static' ? "Expire Date" : "Full unlock Date" }}<sup v-if="voucherType !== 'static'">*</sup>
                    </label>
                  </div>
                  <div class="col-6">
                    <b-form-input type="date" placeholder="Expire Date" v-model="voucherExpire"></b-form-input>
                  </div>
                </div>
                <div class="row">
                  <div class="col-5 text-right">
                    <label class="form-check-label pt-2">NEAR Amount<sup>*</sup></label>
                  </div>
                  <div class="col-6">
                    <b-input-group>
                      <b-form-input type="number" required min="0.1" max="10" step="0.1" v-model="voucherDeposit"></b-form-input>
                      <b-input-group-append>
                        <b-button variant="primary" type="submit">CREATE</b-button>
                      </b-input-group-append>
                    </b-input-group>
                  </div>
                </div>

              </div>
            </form>
          </div>
        </div>
      </div>
      <div v-if="!is_ready" class="text-center">
        <img src="../assets/loader.gif" alt="" width="70">
      </div>
    </div>
  </main>
</template>

<script>
import Big from "big.js";
import sha256 from 'crypto-js/sha256';
import QrcodeVue from 'qrcode.vue';
import Header from "./Header";
import { toNearAmount as toNearAmountUtil, dateFormat as dateFormatUtil, isExpired as isExpiredUtil } from '../utils';

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
    Header,
  },
  data: function () {
    return {
      is_ready: false,
      voucherDeposit: null,
      voucherExpire: null,
      createCount: 1,
      voucherType: 'static',
      vouchers: [],
      voucherTypeOptions: [
        { value: 'static', text: 'Unlocked' },
        { value: 'linear', text: 'Linear Unlock' },
      ],
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    }
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
      let createCount = parseInt(this.createCount) || 1;

      if (this.voucherDeposit <= 0 || createCount < 1) {
        alert("Please specify Deposit");
        return false;
      }

      if (this.voucherType !== 'static' && !this.voucherExpire) {
        alert("Please specify Full unlock Date");
        return false;
      }

      let expireDate = null;
      if (this.voucherExpire) {
        expireDate = Date.parse(this.voucherExpire) * 1000000;
      }
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      let totalDeposit = this.voucherDeposit * createCount;
      let hashList = [];
      let idList = [];

      for (let i = 0; i < createCount; i++) {
        const newId = this.randomStr(12);
        keys[newId] = this.randomStr(64);
        localStorage.setItem('app-private-keys', JSON.stringify(keys));
        hashList.push(sha256(keys[newId]).toString())
        idList.push(newId);
      }

      const GAS = Big(300).times(10 ** 12).toFixed();
      const DEPOSIT = Big(totalDeposit).times(10 ** 24).toFixed();
      try {
        await window.contract.add_voucher({
          id_list: idList,
          hash_list: hashList,
          expire_date: expireDate,
          payment_type: this.voucherType
        }, GAS, DEPOSIT);
      } catch (e) {
        alert("Something went wrong!");
        throw e //re-throw
      } finally {
        this.getVouchers();
      }

    },

    async removeVoucher(id) {
      if (confirm("Please confirm voucher removing.")) {
        const GAS = Big(100).times(10 ** 12).toFixed();
        try {
          await window.contract.remove_voucher({ id }, GAS, 1);
        } catch (e) {
          alert("Something went wrong!");
          throw e //re-throw
        } finally {
          console.log('removed');
          this.getVouchers();
        }
      }
    },

    toNearAmount(amount) {
      return toNearAmountUtil(amount);
    },

    dateFormat(date) {
      return dateFormatUtil(date);
    },

    isExpired(timestamp) {
      return isExpiredUtil(timestamp);
    },

    getUrl(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return window.location.origin + `/?user=${window.accountId}&id=${id}#${keys[id]}`;
    },

    isValidVoucher(id) {
      let keys = JSON.parse(localStorage.getItem('app-private-keys'));
      return keys[id] !== undefined;
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

    print() {
      window.print();
    }

  },
}
</script>
