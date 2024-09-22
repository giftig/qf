package com.xantoria.mmo.common.model.updates

// N.B. not used in this file, just added for testing as an example of a multi import search
import scala.concurrent.{ExecutionContext, Future}

case class StatSheetUpdate(absolute: Boolean, values: StatSheetUpdate.Values) extends Update

object StatSheetUpdate {
  /**
   * A diff of the stat sheet or the new values, depending on update mode
   *
   * Every stat is provided optionally as you may not want to set a new value at all when
   * specifying an absolute update. When specifying a diff update, providing None is equivalent to
   * providing 0
   */
  case class Values(
    hp: Option[Int],
    mp: Option[Int],
    physAtk: Option[Int],
    physDef: Option[Int],
    magAtk: Option[Int],
    magDef: Option[Int],
    speed: Option[Int]
  ) {
    override def toString: String = {
      s"$hp/$mp/$physAtk/$physDef/$magAtk/$magDef/$speed"
    }
  }
}

