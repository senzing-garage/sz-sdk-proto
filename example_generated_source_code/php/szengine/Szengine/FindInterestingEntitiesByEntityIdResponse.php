<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: szengine.proto

namespace Szengine;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>szengine.FindInterestingEntitiesByEntityIdResponse</code>
 */
class FindInterestingEntitiesByEntityIdResponse extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>string result = 1;</code>
     */
    protected $result = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $result
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Szengine::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>string result = 1;</code>
     * @return string
     */
    public function getResult()
    {
        return $this->result;
    }

    /**
     * Generated from protobuf field <code>string result = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setResult($var)
    {
        GPBUtil::checkString($var, True);
        $this->result = $var;

        return $this;
    }

}

