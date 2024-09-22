package com.xantoria.mmo.common.updates.inventory

import com.xantoria.mmo.common.model.inventory._
import com.xantoria.mmo.common.model.item.ItemId
import com.xantoria.mmo.common.model.updates.InventoryUpdate

/**
 * The result of an inventory update being processed, including details of exactly what happened
 */
sealed trait InventoryUpdateResult[+I <: Inventory] {
  val inventory: I
  val effectiveUpdate: Option[InventoryUpdate]
  val rejections: List[InventoryUpdateResult.Rejection]
}

object InventoryUpdateResult {
  sealed trait Rejection
  case object InventoryFull extends Rejection
  case class ItemNotFound(expected: ItemQuantity, actual: ItemQuantity) extends Rejection
  case class ItemStackSizeExceeded(item: ItemId, size: Int) extends Rejection

  /**
   * An update was at least partially applied, but there may have been some issues
   *
   * If the update was requested as strict, a [[Rejected]] result should occur instead if there
   * would be any rejections.
   *
   * Examples:
   *
   *  - adding 10 feathers to an inventory with no slots but which has 5 remaining capacity in
   *    a feather slot results in 5 feathers being added to the inventory, with
   *    effectiveUpdate = Some(AddItem(ItemQuantity(feather, 5), false)),
   *    rejections = ItemStackSizeExceeded(feather)
   *
   *  - removing 10 marbles when only 5 are present results in all 5 being removed, with
   *    effectiveUpdate = Some(AddItem(ItemQuantity(marble, -5), false)),
   *    rejections = ItemNotFound(ItemQuantity(marble, 10), ItemQuantity(marble, 5))
   *
   *  - removing an iron sword where none is present results in no change, with
   *    effectiveUpdate = None,
   *    rejections = ItemNotFound(ItemQuantity(iron_sword, 1), ItemQuantity(iron_sword, 0))
   *
   *  - adding a dragon egg to an empty inventory results in the item being added, with
   *    effectiveUpdate = (original update)
   *    rejections = Nil
   */
  case class Updated[+I <: Inventory](
    inventory: I,
    effectiveUpdate: Option[InventoryUpdate],
    rejections: List[Rejection]
  ) extends InventoryUpdateResult[I]

  case class Rejected[+I <: Inventory](
    inventory: I,
    rejections: List[Rejection]
  ) extends InventoryUpdateResult[I] {

    override val effectiveUpdate: Option[InventoryUpdate] = None
  }
}
