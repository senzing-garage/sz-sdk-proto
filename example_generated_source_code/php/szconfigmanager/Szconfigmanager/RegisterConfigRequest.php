<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: szconfigmanager.proto

namespace Szconfigmanager;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>szconfigmanager.RegisterConfigRequest</code>
 */
class RegisterConfigRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>string config_definition = 1;</code>
     */
    protected $config_definition = '';
    /**
     * Generated from protobuf field <code>string config_comment = 2;</code>
     */
    protected $config_comment = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $config_definition
     *     @type string $config_comment
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Szconfigmanager::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>string config_definition = 1;</code>
     * @return string
     */
    public function getConfigDefinition()
    {
        return $this->config_definition;
    }

    /**
     * Generated from protobuf field <code>string config_definition = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setConfigDefinition($var)
    {
        GPBUtil::checkString($var, True);
        $this->config_definition = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string config_comment = 2;</code>
     * @return string
     */
    public function getConfigComment()
    {
        return $this->config_comment;
    }

    /**
     * Generated from protobuf field <code>string config_comment = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setConfigComment($var)
    {
        GPBUtil::checkString($var, True);
        $this->config_comment = $var;

        return $this;
    }

}

