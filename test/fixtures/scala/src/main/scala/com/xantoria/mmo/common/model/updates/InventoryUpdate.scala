package com.xantoria.mmo.common.model.updates

import com.xantoria.mmo.common.model.inventory.ItemQuantity

sealed trait InventoryUpdate extends Update {
  // If strict, a non-exact update isn't allowed (e.g. remove 12 items from a stack of 10), add
  // some items when others don't fit)
  val strict: Boolean
}

object InventoryUpdate {
  case class AddItem(upd: ItemQuantity, strict: Boolean) extends InventoryUpdate
  case class SetItemQuantity(upd: ItemQuantity, strict: Boolean) extends InventoryUpdate
  case class Multiple(updates: List[InventoryUpdate], strict: Boolean) extends InventoryUpdate
}
