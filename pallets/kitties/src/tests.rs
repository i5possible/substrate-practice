use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn should_create_kitty_successfully() {
	// new_test_ext().execute_with(|| {
	// 	assert_ok!(KittiesModule::create(Origin::signed(1)));
	// 	assert_eq!(KittiesCount::<Test>::get(), 1);
	// });
}

#[test]
fn should_return_error_for_create_when_kitty_index_overflow() {}

#[test]
fn should_transfer_kitty_successfully() {}

#[test]
fn should_return_error_for_transfer_when_not_owner() {}

#[test]
fn should_breed_kitty_successfully() {}

#[test]
fn should_return_error_for_breed_when_same_parent_index() {}

#[test]
fn should_return_error_for_breed_when_invalid_kitty_index() {}

#[test]
fn should_return_error_for_breed_when_kitty_index_overflow() {}
