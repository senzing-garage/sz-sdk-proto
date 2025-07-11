<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: szdiagnostic.proto

namespace Szdiagnostic;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>szdiagnostic.CheckRepositoryPerformanceRequest</code>
 */
class CheckRepositoryPerformanceRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>int32 seconds_to_run = 1;</code>
     */
    protected $seconds_to_run = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int $seconds_to_run
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Szdiagnostic::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>int32 seconds_to_run = 1;</code>
     * @return int
     */
    public function getSecondsToRun()
    {
        return $this->seconds_to_run;
    }

    /**
     * Generated from protobuf field <code>int32 seconds_to_run = 1;</code>
     * @param int $var
     * @return $this
     */
    public function setSecondsToRun($var)
    {
        GPBUtil::checkInt32($var);
        $this->seconds_to_run = $var;

        return $this;
    }

}

