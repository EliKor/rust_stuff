use std::cmp::{max, min};

// Given a list of prices and the ability to
// buy and sell only once, what is the max profit you can make
pub fn stocks_i(prices: Vec<i32>) -> i32 {
  let mut max_profit = 0;
  let mut min_seen = prices[0];
  for price in prices {
    min_seen = min(min_seen, price);
    max_profit = max(max_profit, price - min_seen);
  }
  max_profit
}

// Given the ability to buy and sell an unlimited
// number of times, what is the max profit you can make
pub fn stocks_ii(prices: Vec<i32>) -> i32 {
  let mut profit = 0;
  for i in 0..prices.len() - 1 {
    if prices[i] < prices[i + 1] {
      profit += prices[i + 1] - prices[i];
    }
  }
  profit
}

// Given the ability to buy and sell twice
// what is the max profit you can make
pub fn stocks_iii(prices: Vec<i32>) -> i32 {
  let mut cost_of_first_buy = prices[0];
  let mut profit_from_first_sell = 0;
  let mut cost_of_second_buy = prices[0];
  let mut profit_from_second_sell = 0;
  for price in prices {
    // first_buy and first_sell are just like for
    // stocks_i solution
    cost_of_first_buy = min(cost_of_first_buy, price);
    profit_from_first_sell = max(profit_from_first_sell, price - cost_of_first_buy);

    // for the second buy and second sell,
    // we either have these equal to first_buy, first_sell
    // if we only have a single buy/sell of stock
    // or we have second_buy be negative to represent
    // the profit left over from the first sell
    cost_of_second_buy = min(cost_of_second_buy, price - profit_from_first_sell);
    profit_from_second_sell = max(profit_from_second_sell, price - cost_of_second_buy);
  }
  profit_from_second_sell
}

// Given the ability to buy and sell unlimited
// times but with a one day cooldown after selling
// what is the max profit you can make
pub fn stocks_with_cooldown(prices: Vec<i32>) -> i32 {
  let n = prices.len();
  if n < 2 {
    return 0;
  }
  let mut profit_with_stock = vec![0; n];
  let mut profit_without_stock = vec![0; n];
  profit_with_stock[0] = -prices[0];
  for i in 1..prices.len() {
    // Coming out of day i you have 2 options
    // 1. You end day i owning the stock
    // 2. You end day i not owning the stock
    // First consider ending the day owning the stock
    // We either buy the stock today (having sold at least 2 days prior)
    // Or we hold onto the stock having already bought the stock
    let buy_profit = if i > 1 {
      // We are past the first 2 days and so if we already sold the stock
      // we can buy again
      profit_without_stock[i - 2] - prices[i]
    } else {
      // We are still within the first two days so we either hold or buy now
      max(profit_with_stock[i - 1], -prices[i])
    };
    profit_with_stock[i] = max(profit_with_stock[i - 1], buy_profit);

    profit_without_stock[i] = max(
      // we don't sell the stock because we don't own it
      profit_without_stock[i - 1],
      // we sell the stock having bought it some day prior
      prices[i] + profit_with_stock[i - 1],
    );
  }
  // The optimal solution will not end with the stock being owned
  // since we would lose profit overall by buying the stock and not selling it
  // So we return the profit without owning the stock on the last day
  profit_without_stock[n - 1]
}

pub fn stocks_iv_slowest(k: i32, prices: Vec<i32>) -> i32 {
  if prices.len() < 2 || k < 1 {
    return 0;
  }
  let mut dp = vec![vec![0; prices.len()]; k as usize + 1];
  for transaction_count in 1..k as usize + 1 {
    for sell_idx in 1..prices.len() {
      for buy_idx in 0..sell_idx {
        let max_profit_before_buying = if buy_idx > 0 {
          dp[transaction_count - 1][buy_idx - 1]
        } else {
          0
        };
        let profit_from_selling_now = prices[sell_idx] - prices[buy_idx];
        let total_profit_from_selling = max_profit_before_buying + profit_from_selling_now;
        let profit_from_not_selling = dp[transaction_count][sell_idx - 1];
        let profit = max(total_profit_from_selling, profit_from_not_selling);
        dp[transaction_count][sell_idx] = max(dp[transaction_count][sell_idx], profit)
      }
    }
  }
  dp[k as usize][prices.len() - 1]
}

pub fn stocks_iv_slower(k: i32, prices: Vec<i32>) -> i32 {
  if prices.len() < 2 || k < 1 {
    return 0;
  }
  let mut dp = vec![vec![0; prices.len()]; k as usize + 1];
  for transaction_count in 1..k as usize + 1 {
    let mut prev_max_transaction_profit = -prices[0];
    for sell_idx in 1..prices.len() {
      let profit_from_not_selling = dp[transaction_count][sell_idx - 1];
      let profit_from_selling = prices[sell_idx] + prev_max_transaction_profit;
      dp[transaction_count][sell_idx] = max(profit_from_not_selling, profit_from_selling);
      prev_max_transaction_profit = max(
        prev_max_transaction_profit,
        dp[transaction_count - 1][sell_idx - 1] - prices[sell_idx], // total profit if buying at sell_idx
      );
    }
  }
  dp[k as usize][prices.len() - 1]
}

#[cfg(test)]
mod test {
  use super::*;

  // stocks_i tests
  #[test]
  fn i_test_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(stocks_i(prices), 5);
  }
  #[test]
  fn i_test_2() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(stocks_i(prices), 0);
  }

  // stocks_ii tests
  #[test]
  fn ii_test_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(stocks_ii(prices), 7);
  }
  #[test]
  fn ii_test_2() {
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(stocks_ii(prices), 4);
  }
  #[test]
  fn ii_test_3() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(stocks_ii(prices), 0);
  }
  // stocks_iii tests
  #[test]
  fn iii_test_1() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    assert_eq!(stocks_iii(prices), 6);
  }
  #[test]
  fn iii_test_2() {
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(stocks_iii(prices), 4);
  }
  #[test]
  fn iii_test_3() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(stocks_iii(prices), 0);
  }
  #[test]
  fn iii_test_4() {
    let prices = vec![2, 1, 2, 0, 1];
    assert_eq!(stocks_iii(prices), 2);
  }
}
