module.exports = ({ wallets, refs, config, client }) => ({
  getHotel: (name) => client.query("hotel", { get_hotel: {name} }),
  createHotel: (signer = wallets.validator,name,rooms,price_per_day) =>
    client.execute(signer, "hotel", { create_hotel: {name,rooms,price_per_day} }),
  takeRoom: (signer = wallets.validator,hotel_name,days) =>
    client.execute(signer, "hotel", { take_room: {hotel_name,days} },{"uluna": 1000000}),
  takeFunds: (signer = wallets.validator,hotel_name) =>
    client.execute(signer, "hotel", { take_funds: {hotel_name} }),
});
